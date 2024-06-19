use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use crate::database::{get_connection, PgPool};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use protos::event::v1::{CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse};
use protos::event::v1::closure_service_server::ClosureService;
use crate::server::services::v1::closure::closure_handlers::{create_closure, delete_closure, list_closures, update_closure};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct ClosureServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: Arc<redis::Client>,
}

impl Clone for ClosureServiceServerImpl {
    fn clone(&self) -> Self {
        ClosureServiceServerImpl {
            pool: Arc::clone(&self.pool),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl ClosureServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: Arc<redis::Client>) -> Self {
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
        create_closure(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn list_closures(&self, request: Request<ListClosuresRequest>) -> Result<Response<ListClosuresResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        list_closures(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn update_closure(&self, request: Request<UpdateClosureRequest>) -> Result<Response<UpdateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        update_closure(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn delete_closure(&self, request: Request<DeleteClosureRequest>) -> Result<Response<DeleteClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        delete_closure(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }
}
