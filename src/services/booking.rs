use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use protos::booking::v1::{booking_service_server::BookingService, CreateBookingRequest, CreateBookingResponse, CreateEventRequest, CreateEventResponse, CreateClosureRequest, CreateClosureResponse, GetBookingRequest, GetBookingResponse, GetEventRequest, GetEventResponse, ListEventsRequest, ListEventsResponse, UpdateEventRequest, UpdateEventResponse, DeleteEventRequest, DeleteEventResponse, DeleteBookingRequest, DeleteBookingResponse, UpdateClosureRequest, UpdateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListBookingsRequest, ListBookingsResponse, ListClosuresRequest, ListClosuresResponse};
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

    async fn update_event(&self, request: Request<UpdateEventRequest>) -> Result<Response<UpdateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::update_event(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn delete_event(&self, request: Request<DeleteEventRequest>) -> Result<Response<DeleteEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::delete_event(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn list_events(&self, request: Request<ListEventsRequest>) -> Result<Response<ListEventsResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::list_events(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn create_booking(&self, request: Request<CreateBookingRequest>) -> Result<Response<CreateBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_booking(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_booking(&self, request: Request<GetBookingRequest>) -> Result<Response<GetBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::get_booking_by_id(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn delete_booking(&self, request: Request<DeleteBookingRequest>) -> Result<Response<DeleteBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::delete_booking(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn list_bookings(&self, request: Request<ListBookingsRequest>) -> Result<Response<ListBookingsResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::list_bookings(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn create_closure(&self, request: Request<CreateClosureRequest>) -> Result<Response<CreateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_closure(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn update_closure(&self, request: Request<UpdateClosureRequest>) -> Result<Response<UpdateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::update_closure(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn delete_closure(&self, request: Request<DeleteClosureRequest>) -> Result<Response<DeleteClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::delete_closure(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn list_closures(&self, request: Request<ListClosuresRequest>) -> Result<Response<ListClosuresResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::list_closures(request.into_inner(), &mut conn).map(Response::new)
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
