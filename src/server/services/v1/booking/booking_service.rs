use autometrics::autometrics;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::database::{get_connection, CacheClient, PgPool};

use crate::server::services::v1::booking::booking_handlers::{
    create_booking, delete_booking, get_booking_by_id, list_bookings,
};
use crate::utils::request_wrapper::RequestMetadata;
use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
use event_protos::event::v1::booking_service_server::BookingService;
use event_protos::event::v1::{
    CreateBookingRequest, CreateBookingResponse, DeleteBookingRequest, DeleteBookingResponse,
    GetBookingRequest, GetBookingResponse, ListBookingsRequest, ListBookingsResponse,
};

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
        BookingServiceServerImpl { pool, cache }
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl BookingService for BookingServiceServerImpl {
    async fn create_booking(
        &self,
        request: Request<CreateBookingRequest>,
    ) -> Result<Response<CreateBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<CreateBookingRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        create_booking(
            request_metadata.request,
            request_metadata.metadata,
            &mut conn,
        )
        .await
        .map(Response::new)
        .map_err(|e| e.into())
    }

    async fn get_booking(
        &self,
        request: Request<GetBookingRequest>,
    ) -> Result<Response<GetBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<GetBookingRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        get_booking_by_id(
            request_metadata.request,
            request_metadata.metadata,
            &mut conn,
        )
        .await
        .map(Response::new)
        .map_err(|e| e.into())
    }

    async fn list_bookings(
        &self,
        request: Request<ListBookingsRequest>,
    ) -> Result<Response<ListBookingsResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;

        let request_metadata: RequestMetadata<ListBookingsRequest> = RequestMetadata {
            metadata: &request.metadata().clone(),
            request: request.into_inner(),
        };

        list_bookings(
            request_metadata.request,
            request_metadata.metadata,
            &mut conn,
        )
        .await
        .map(Response::new)
        .map_err(|e| e.into())
    }

    async fn delete_booking(
        &self,
        request: Request<DeleteBookingRequest>,
    ) -> Result<Response<DeleteBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool).await?;
        delete_booking(request.into_inner(), &mut conn)
            .await
            .map(Response::new)
            .map_err(|e| e.into())
    }
}
