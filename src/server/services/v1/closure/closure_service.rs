use autometrics::autometrics;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::database::{get_connection, CacheClient, PgPool};

use crate::server::services::v1::closure::closure_handlers::{
    create_closure, delete_closure, list_closures, update_closure,
};
use crate::utils::request_wrapper::RequestMetadata;
use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
use event_protos::event::v1::closure_service_server::ClosureService;
use event_protos::event::v1::{
    CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse,
    ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse,
};
use crate::Config;

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct ClosureServiceServerImpl {
    pub cfg: Config,
    pub pool: Arc<PgPool>,
    pub cache: Arc<Option<CacheClient>>
}

impl Clone for ClosureServiceServerImpl {
    fn clone(&self) -> Self {
        ClosureServiceServerImpl {
            cfg: self.cfg.clone(),
            pool: Arc::clone(&self.pool),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl ClosureServiceServerImpl {
    pub(crate) fn new(cfg: Config, pool: Arc<PgPool>, cache: Arc<Option<CacheClient>>) -> Self {
        ClosureServiceServerImpl { cfg, pool, cache }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl ClosureService for ClosureServiceServerImpl {
    async fn create_closure(
        &self,
        request: Request<CreateClosureRequest>,
    ) -> Result<Response<CreateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<CreateClosureRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let response = create_closure(
            request_metadata.request,
            request_metadata.metadata,
            &mut conn,
        )
        .await
        .map(Response::new)?;

        if let Some(cache) = &*self.cache {
            let inner_response = response.get_ref();
            cache
                .invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key)
                .await?;
        }

        Ok(response)
    }

    async fn list_closures(
        &self,
        request: Request<ListClosuresRequest>,
    ) -> Result<Response<ListClosuresResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<ListClosuresRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let list_event = {
            let request_metadata = request_metadata.clone();
            move || async move {
                list_closures(
                    request_metadata.request,
                    request_metadata.metadata,
                    &mut conn,
                )
                    .await
                    .map(Response::new)
                    .map_err(|e| e.into())
            }
        };

        match &*self.cache {
            Some(cache) => {
                cache.handle_cache("get_event", &request_metadata, list_event).await
            }
            None => list_event().await,
        }
    }

    async fn update_closure(
        &self,
        request: Request<UpdateClosureRequest>,
    ) -> Result<Response<UpdateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<UpdateClosureRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        let response = update_closure(
            request_metadata.request,
            request_metadata.metadata,
            &mut conn,
        )
        .await
        .map(Response::new)?;

        if let Some(cache) = &*self.cache {
            let inner_response = response.get_ref();
            cache
                .invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key)
                .await?;
        }


        Ok(response)
    }

    async fn delete_closure(
        &self,
        request: Request<DeleteClosureRequest>,
    ) -> Result<Response<DeleteClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        let response = delete_closure(inner_request.clone(), &mut conn)
            .await
            .map(Response::new)?;

        // TODO: Find a way to invalidate cache keys
        // let inner_response = response.get_ref();
        // self.cache.invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key).await?;

        Ok(response)
    }
}
