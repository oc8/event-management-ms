use std::sync::{Arc};
use autometrics::autometrics;
use tonic::{Request, Response, Status};

use protos::booking::v1::{booking_service_server::BookingService, CreateBookingRequest, CreateBookingResponse, CreateEventRequest, CreateEventResponse, CreateClosureRequest, CreateClosureResponse, GetBookingRequest, GetBookingResponse, GetEventRequest, GetEventResponse, ListEventsRequest, ListEventsResponse, UpdateEventRequest, UpdateEventResponse, DeleteEventRequest, DeleteEventResponse, DeleteBookingRequest, DeleteBookingResponse, UpdateClosureRequest, UpdateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListBookingsRequest, ListBookingsResponse, ListClosuresRequest, ListClosuresResponse, CancelEventRequest, CancelEventResponse};
use crate::database::{PgPool, PgPooledConnection};
use crate::{errors, rpcs};

use autometrics::objectives::{
    Objective, ObjectiveLatency, ObjectivePercentile
};
use redis::{Commands};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;
use booking_ms::{report_error};
use crate::errors::{ApiError, format_error};

const API_SLO: Objective = Objective::new("api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms250, ObjectivePercentile::P99);

pub struct BookingServiceServerImpl {
    pub pool: Arc<PgPool>,
    pub cache: Arc<RwLock<redis::Client>>,
}

impl Clone for BookingServiceServerImpl {
    fn clone(&self) -> Self {
        BookingServiceServerImpl {
            pool: self.pool.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl BookingServiceServerImpl {
    pub(crate) fn new(pool: Arc<PgPool>, cache: redis::Client) -> Self {
        BookingServiceServerImpl {
            pool,
            cache: Arc::new(RwLock::new(cache)),
        }
    }

    fn generate_cache_key(&self, method_name: &str, request: &impl Serialize) -> String {
        format!("{}:{}", method_name, serde_json::to_string(request).unwrap())
    }

    async fn get_cache<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Status> {
        let cache = self.cache.read().await;
        let mut conn = cache.get_connection().map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        let data: Option<Vec<u8>> = conn.get(key).map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        if let Some(data) = data {
            let result = serde_json::from_slice(&data).map_err(|e| {
                report_error(&e);
                format_error(ApiError {
                    grpc_code: tonic::Code::Internal,
                    code: "deserialization_error",
                    message: "Failed to deserialize cached data",
                })
            })?;

            log::debug!("Cache hit for key: {}", key);

            Ok(Some(result))
        } else {

            log::debug!("Cache miss for key: {}", key);

            Ok(None)
        }
    }

    async fn set_cache<T: Serialize>(&self, key: &str, value: &T) -> Result<(), Status> {
        let cache_ttl = std::env::var("CACHE_TTL")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u64>()
            .unwrap();

        let cache = self.cache.read().await;
        let mut conn = cache.get_connection().map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        let data = serde_json::to_vec(value).map_err(|e| {
            report_error(&e);
            format_error(ApiError {
                grpc_code: tonic::Code::Internal,
                code: "serialization_error",
                message: "Failed to serialize response",
            })
        })?;

        conn.set_ex(key, data, cache_ttl).map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        log::debug!("Cache set for key: {}", key);

        Ok(())
    }

    async fn invalid_cache(&self, method_name: &str, request: &impl Serialize) -> Result<(), Status> {
        let cache_key = self.generate_cache_key(method_name, request);
        let cache = self.cache.read().await;
        let mut conn = cache.get_connection().map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        conn.del(cache_key).map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        Ok(())
    }

    async fn invalidate_related_cache_keys(&self, organizer_key: String) -> Result<(), Status> {
        let cache = self.cache.read().await;
        let mut conn = cache.get_connection().map_err(|e| {
            report_error(&e);
            format_error(errors::REDIS_CONNECTION_FAILURE)
        })?;

        let keys_to_invalidate = vec![
            "list_*:{\"filters\":{*\"organizerKey\":\"".to_string() + &organizer_key + "\"*}*",
        ];

        log::debug!("Invalidating cache keys: {:?}", keys_to_invalidate);

        for key_pattern in keys_to_invalidate {
            let keys: Vec<String> = conn.keys(key_pattern).map_err(|e| {
                report_error(&e);
                format_error(errors::REDIS_CONNECTION_FAILURE)
            })?;
            for key in keys {
                conn.del(&key).map_err(|e| {
                    report_error(&e);
                    format_error(errors::REDIS_CONNECTION_FAILURE)
                })?;
            }
        }

        Ok(())
    }

    async fn handle_cache<T, F>(
        &self,
        method_name: &str,
        request: &impl Serialize,
        call: F,
    ) -> Result<Response<T>, Status>
        where
            T: DeserializeOwned + Serialize,
            F: FnOnce() -> Result<Response<T>, Status> + Send,
    {
        let cache_key = self.generate_cache_key(method_name, request);
        if let Some(cached_response) = self.get_cache::<T>(&cache_key).await? {
            return Ok(Response::new(cached_response));
        }

        let response = call()?;

        self.set_cache(&cache_key, response.get_ref()).await?;
        Ok(response)
    }
}

#[tonic::async_trait]
#[autometrics(objective = API_SLO)]
impl BookingService for BookingServiceServerImpl {
    async fn create_event(&self, request: Request<CreateEventRequest>) -> Result<Response<CreateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let request_inner = request.into_inner();

        let response = rpcs::create_event(request_inner.clone(), &mut conn).map(Response::new)?;

        Ok(response)
    }

    async fn get_event(&self, request: Request<GetEventRequest>) -> Result<Response<GetEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let request_inner = request.into_inner();

        self.handle_cache("get_event", &request_inner, || {
            rpcs::get_event_by_id(request_inner.clone(), &mut conn).map(Response::new)
        }).await
    }

    async fn update_event(&self, request: Request<UpdateEventRequest>) -> Result<Response<UpdateEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let request_inner = request.into_inner();

        let response = rpcs::update_event(request_inner.clone(), &mut conn).map(Response::new)?;

        self.invalid_cache("get_event", &GetEventRequest {
            id: request_inner.id.clone(),
        }).await?;

        let response_inner = response.get_ref();
        self.invalidate_related_cache_keys(response_inner.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn delete_event(&self, request: Request<DeleteEventRequest>) -> Result<Response<DeleteEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.invalid_cache("get_event", &GetEventRequest {
            id: inner_request.id.clone(),
        }).await?;

        rpcs::delete_event(inner_request, &mut conn).map(Response::new)
    }

    async fn list_events(&self, request: Request<ListEventsRequest>) -> Result<Response<ListEventsResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let request_inner = request.into_inner();

        self.handle_cache("list_events", &request_inner, || {
            rpcs::list_events(request_inner.clone(), &mut conn).map(Response::new)
        }).await
    }

    async fn cancel_event(&self, request: Request<CancelEventRequest>) -> Result<Response<CancelEventResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let request_inner = request.into_inner();

        self.invalid_cache("get_event", &GetEventRequest {
            id: request_inner.id.clone(),
        }).await?;

        let response = rpcs::cancel_event(request_inner.clone(), &mut conn).map(Response::new)?;

        let response_inner = response.get_ref();
        self.invalidate_related_cache_keys(response_inner.clone().event.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn create_booking(&self, request: Request<CreateBookingRequest>) -> Result<Response<CreateBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_booking(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn get_booking(&self, request: Request<GetBookingRequest>) -> Result<Response<GetBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.handle_cache("get_booking", &inner_request, || {
            rpcs::get_booking_by_id(inner_request.clone(), &mut conn).map(Response::new)
        }).await
    }

    async fn delete_booking(&self, request: Request<DeleteBookingRequest>) -> Result<Response<DeleteBookingResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.invalid_cache("get_booking", &GetBookingRequest {
            id: inner_request.id.clone(),
        }).await?;

        rpcs::delete_booking(inner_request.clone(), &mut conn).map(Response::new)
    }

    async fn list_bookings(&self, request: Request<ListBookingsRequest>) -> Result<Response<ListBookingsResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.handle_cache("list_bookings", &inner_request, || {
            rpcs::list_bookings(inner_request.clone(), &mut conn).map(Response::new)
        }).await
    }

    async fn create_closure(&self, request: Request<CreateClosureRequest>) -> Result<Response<CreateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        rpcs::create_closure(request.into_inner(), &mut conn).map(Response::new)
    }

    async fn update_closure(&self, request: Request<UpdateClosureRequest>) -> Result<Response<UpdateClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.invalid_cache("get_event", &GetEventRequest {
            id: inner_request.id.clone(),
        }).await?;

        let response = rpcs::update_closure(inner_request.clone(), &mut conn).map(Response::new)?;

        let response_inner = response.get_ref();
        self.invalidate_related_cache_keys(response_inner.clone().closure.unwrap().organizer_key).await?;

        Ok(response)
    }

    async fn delete_closure(&self, request: Request<DeleteClosureRequest>) -> Result<Response<DeleteClosureResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.invalid_cache("get_event", &GetEventRequest {
            id: inner_request.id.clone(),
        }).await?;

        rpcs::delete_closure(inner_request.clone(), &mut conn).map(Response::new)
    }

    async fn list_closures(&self, request: Request<ListClosuresRequest>) -> Result<Response<ListClosuresResponse>, Status> {
        let mut conn = get_connection(&self.pool)?;
        let inner_request = request.into_inner();

        self.handle_cache("list_closures", &inner_request, || {
            rpcs::list_closures(inner_request.clone(), &mut conn).map(Response::new)
        }).await
    }
}

pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    match pool.get() {
        Err(_) => Err(format_error(errors::DATABASE_CONNECTION_FAILURE)),
        Ok(conn) => Ok(conn),
    }
}
