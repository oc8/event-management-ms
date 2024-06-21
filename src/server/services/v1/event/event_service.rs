use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use crate::database::{get_connection, PgPool};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use protos::event::v1::{CancelEventRequest, CancelEventResponse, CreateEventRequest, CreateEventResponse, DeleteEventRequest, DeleteEventResponse, GetEventRequest, GetEventResponse, ListEventsRequest, ListEventsResponse, GetTimelineRequest, GetTimelineResponse, UpdateEventRequest, UpdateEventResponse};
use protos::event::v1::event_service_server::EventService;
use crate::server::services::v1::event::event_handlers::{cancel_event, create_event, delete_event, get_event_by_id, get_timeline, list_events, update_event};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct EventServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: Arc<redis::Client>,
}

impl Clone for EventServiceServerImpl {
    fn clone(&self) -> Self {
        EventServiceServerImpl {
            pool: Arc::clone(&self.pool),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl EventServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: Arc<redis::Client>) -> Self {
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
        create_event(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn get_event(&self, request: Request<GetEventRequest>) -> Result<Response<GetEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        get_event_by_id(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn list_events(&self, request: Request<ListEventsRequest>) -> Result<Response<ListEventsResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        list_events(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn update_event(&self, request: Request<UpdateEventRequest>) -> Result<Response<UpdateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        update_event(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn delete_event(&self, request: Request<DeleteEventRequest>) -> Result<Response<DeleteEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        delete_event(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn cancel_event(&self, request: Request<CancelEventRequest>) -> Result<Response<CancelEventResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        cancel_event(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn get_timeline(&self, request: Request<GetTimelineRequest>) -> Result<Response<GetTimelineResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        get_timeline(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }
}
