use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use crate::database::{CacheClient, get_connection, PgPool};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use protos::event::v1::{CancelEventRequest, CancelEventResponse, CreateEventRequest, CreateEventResponse, DeleteEventRequest, DeleteEventResponse, GetEventRequest, GetEventResponse, ListEventsRequest, ListEventsResponse, GetTimelineRequest, GetTimelineResponse, UpdateEventRequest, UpdateEventResponse};
use protos::event::v1::event_service_server::EventService;
use crate::server::services::v1::event::event_handlers::{cancel_event, create_event, delete_event, get_event_by_id, get_timeline, list_events, update_event};
use crate::utils::request_wrapper::RequestMetadata;

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct EventServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: CacheClient,
}

impl Clone for EventServiceServerImpl {
    fn clone(&self) -> Self {
        EventServiceServerImpl {
            pool: Arc::clone(&self.pool),
            cache: self.cache.clone(),
        }
    }
}

impl EventServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: CacheClient) -> Self {
        EventServiceServerImpl {
            pool,
            cache,
        }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl EventService for EventServiceServerImpl {
    async fn create_event(&self, request: Request<CreateEventRequest>) -> Result<Response<CreateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<CreateEventRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let response = create_event(request_metadata.request, request_metadata.metadata, &mut conn)
            .await
            .map(Response::new)?;

        let inner_response = response.get_ref();
        self.cache.invalidate_related_cache_keys(inner_response.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn get_event(&self, request: Request<GetEventRequest>) -> Result<Response<GetEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<GetEventRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        self.cache.handle_cache("get_event", &request_metadata.clone(), || {
            async move {
                get_event_by_id(request_metadata.request, request_metadata.metadata, &mut conn)
                    .await
                    .map(Response::new)
                    .map_err(|e| e.into())
            }
        }).await
    }

    async fn list_events(&self, request: Request<ListEventsRequest>) -> Result<Response<ListEventsResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<ListEventsRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        self.cache.handle_cache("list_events", &request_metadata.clone(), || {
            async move {
                list_events(request_metadata.request, request_metadata.metadata, &mut conn)
                    .await
                    .map(Response::new)
                    .map_err(|e| e.into())
            }
        }).await
    }

    async fn update_event(&self, request: Request<UpdateEventRequest>) -> Result<Response<UpdateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<UpdateEventRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let response = update_event(request_metadata.request.clone(), request_metadata.metadata, &mut conn)
            .await
            .map(Response::new)?;

        self.cache.invalid_cache("get_event", &GetEventRequest {
            id: request_metadata.request.id.clone(),
        }).await?;

        let inner_response = response.get_ref();
        self.cache.invalidate_related_cache_keys(inner_response.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn delete_event(&self, request: Request<DeleteEventRequest>) -> Result<Response<DeleteEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        let response = delete_event(inner_request, &mut conn)
            .await
            .map(Response::new)?;

        // TODO: Find a way to invalidate cache keys
        // let inner_response = response.get_ref();
        // self.cache.invalidate_related_cache_keys(response_inner.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn cancel_event(&self, request: Request<CancelEventRequest>) -> Result<Response<CancelEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<CancelEventRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let response = cancel_event(request_metadata.request.clone(), request_metadata.metadata, &mut conn)
            .await
            .map(Response::new)?;

        self.cache.invalid_cache("get_event", &GetEventRequest {
            id: request_metadata.request.id.clone(),
        }).await?;

        let inner_response = response.get_ref();
        self.cache.invalidate_related_cache_keys(inner_response.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn get_timeline(&self, request: Request<GetTimelineRequest>) -> Result<Response<GetTimelineResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<GetTimelineRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        self.cache.handle_cache("list_events", &request_metadata.clone(), || {
            async move {
                get_timeline(request_metadata.request, request_metadata.metadata, &mut conn)
                    .await
                    .map(Response::new)
                    .map_err(|e| e.into())
            }
        }).await
    }
}
