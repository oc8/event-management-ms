use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use crate::database::{CacheClient, get_connection, PgPool};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use protos::event::v1::{CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse};
use protos::event::v1::closure_service_server::ClosureService;
use crate::errors::ApiError;
use crate::server::services::v1::closure::closure_handlers::{create_closure, delete_closure, list_closures, update_closure};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct ClosureServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: CacheClient,
}

impl Clone for ClosureServiceServerImpl {
    fn clone(&self) -> Self {
        ClosureServiceServerImpl {
            pool: Arc::clone(&self.pool),
            cache: self.cache.clone(),
        }
    }
}

impl ClosureServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: CacheClient) -> Self {
        ClosureServiceServerImpl {
            pool,
            cache,
        }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl ClosureService for ClosureServiceServerImpl {
    async fn create_closure(&self, request: Request<CreateClosureRequest>) -> Result<Response<CreateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        let response = create_closure(inner_request.clone(), &mut conn)
            .await
            .map(Response::new)?;

        let inner_response = response.get_ref();
        self.cache.invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn list_closures(&self, request: Request<ListClosuresRequest>) -> Result<Response<ListClosuresResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        self.cache.handle_cache("list_closures", &inner_request.clone(), || {
            async move {
                list_closures(inner_request, &mut conn)
                    .await
                    .map(Response::new)
                    .map_err(|e| e.into())
            }
        }).await
    }

    async fn update_closure(&self, request: Request<UpdateClosureRequest>) -> Result<Response<UpdateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        let response = update_closure(inner_request, &mut conn)
            .await
            .map(Response::new)?;

        let inner_response = response.get_ref();
        self.cache.invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn delete_closure(&self, request: Request<DeleteClosureRequest>) -> Result<Response<DeleteClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        let inner_request = request.into_inner();

        let response =  delete_closure(inner_request.clone(), &mut conn)
            .await
            .map(Response::new)?;

        // TODO: Find a way to invalidate cache keys
        // let inner_response = response.get_ref();
        // self.cache.invalidate_related_cache_keys(inner_response.clone().closure.unwrap().organizer_key).await?;

        Ok(response)
    }
}
