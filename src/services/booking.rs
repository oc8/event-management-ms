use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Code, Request, Response, Status};

use protos::booking::v1::{booking_service_server::BookingService, CreateEventRequest, CreateEventResponse, GetEventRequest, GetEventResponse};
use crate::database::{PgPool, PgPooledConnection};
use crate::{errors, rpcs};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct BookingServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub r_client: redis::Client,
}

impl Clone for BookingServiceServerImpl {
    fn clone(&self) -> Self {
        BookingServiceServerImpl {
            pool: self.pool.clone(),
            r_client: self.r_client.clone(),
        }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl BookingService for BookingServiceServerImpl {
    async fn create_event(&self, request: Request<CreateEventRequest>) -> Result<Response<CreateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_event(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_event(&self, request: Request<GetEventRequest>) -> Result<Response<GetEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_event_by_id(request.into_inner(), &mut conn).map(Response::new)
    }
}

pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    match pool.get() {
        Err(_) => Err(Status::new(Code::DataLoss, errors::DATABASE_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}

fn get_redis_connection(r_client: &redis::Client) -> Result<redis::Connection, Status> {
    match r_client.get_connection() {
        Err(_) => Err(Status::new(Code::DataLoss, errors::REDIS_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}
