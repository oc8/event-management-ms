use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use protos::booking::v1::{booking_service_server::BookingService, CreateBookingRequest, CreateBookingResponse, CreateEventRequest, CreateEventResponse, CreateClosureRequest, CreateClosureResponse, GetBookingRequest, GetBookingResponse, GetEventRequest, GetEventResponse, GetEventInstancesRequest, GetEventInstancesResponse, GetActiveEventsInstancesRequest, GetActiveEventsInstancesResponse, ListEventsRequest, ListEventsResponse, GetTimelineRequest, GetTimelineResponse};
use crate::database::{PgPool, PgPooledConnection};
use crate::{errors, rpcs};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use crate::errors::format_error;

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

    async fn list_events(&self, request: Request<ListEventsRequest>) -> Result<Response<ListEventsResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::list_events(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_event_instances(&self, request: Request<GetEventInstancesRequest>) -> Result<Response<GetEventInstancesResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_event_instances(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_active_events_instances(&self, request: Request<GetActiveEventsInstancesRequest>) -> Result<Response<GetActiveEventsInstancesResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_active_events_instances(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn create_booking(&self, request: Request<CreateBookingRequest>) -> Result<Response<CreateBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_booking(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_booking(&self, request: Request<GetBookingRequest>) -> Result<Response<GetBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_booking_by_id(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn create_closure(&self, request: Request<CreateClosureRequest>) -> Result<Response<CreateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_closure(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_timeline(&self, request: Request<GetTimelineRequest>) -> Result<Response<GetTimelineResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_timeline(request.into_inner(), &mut conn).map(Response::new)
    }
}

pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    match pool.get() {
        Err(_) => Err(format_error(errors::DATABASE_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}

fn get_redis_connection(r_client: &redis::Client) -> Result<redis::Connection, Status> {
    match r_client.get_connection() {
        Err(_) => Err(format_error(errors::REDIS_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}
