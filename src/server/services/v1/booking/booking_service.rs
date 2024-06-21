use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use crate::database::{CacheClient, get_connection, PgPool};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use protos::event::v1::booking_service_server::BookingService;
use protos::event::v1::{CreateBookingRequest, CreateBookingResponse, DeleteBookingRequest, DeleteBookingResponse, GetBookingRequest, GetBookingResponse, ListBookingsRequest, ListBookingsResponse};
use crate::server::services::v1::booking::booking_handlers::{create_booking, delete_booking, get_booking_by_id, list_bookings};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct BookingServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: CacheClient,
}

impl Clone for BookingServiceServerImpl {
    fn clone(&self) -> Self {
        BookingServiceServerImpl {
            pool: Arc::clone(&self.pool),
            cache: self.cache.clone(),
        }
    }
}

impl BookingServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: CacheClient) -> Self {
        BookingServiceServerImpl {
            pool,
            cache,
        }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl BookingService for BookingServiceServerImpl {
    async fn create_booking(&self, request: Request<CreateBookingRequest>) -> Result<Response<CreateBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        create_booking(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn get_booking(&self, request: Request<GetBookingRequest>) -> Result<Response<GetBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        get_booking_by_id(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn list_bookings(&self, request: Request<ListBookingsRequest>) -> Result<Response<ListBookingsResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        list_bookings(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }

    async fn delete_booking(&self, request: Request<DeleteBookingRequest>) -> Result<Response<DeleteBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        delete_booking(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }
}
