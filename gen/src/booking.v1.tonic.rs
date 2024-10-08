// @generated
/// Generated client implementations.
pub mod booking_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BookingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BookingServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BookingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BookingServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BookingServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/CreateEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "CreateEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/GetEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "GetEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_event(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/UpdateEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "UpdateEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/DeleteEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "DeleteEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/ListEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "ListEvents"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelEventResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/CancelEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "CancelEvent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_booking(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBookingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/CreateBooking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "CreateBooking"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_booking(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBookingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/GetBooking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "GetBooking"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_booking(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBookingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/DeleteBooking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "DeleteBooking"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_bookings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBookingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBookingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/ListBookings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "ListBookings"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_closure(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateClosureResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/CreateClosure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "CreateClosure"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_closure(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateClosureResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/UpdateClosure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "UpdateClosure"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_closure(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteClosureResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/DeleteClosure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "DeleteClosure"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_closures(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClosuresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListClosuresResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/booking.v1.BookingService/ListClosures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("booking.v1.BookingService", "ListClosures"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod booking_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BookingServiceServer.
    #[async_trait]
    pub trait BookingService: Send + Sync + 'static {
        async fn create_event(
            &self,
            request: tonic::Request<super::CreateEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEventResponse>,
            tonic::Status,
        >;
        async fn get_event(
            &self,
            request: tonic::Request<super::GetEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventResponse>,
            tonic::Status,
        >;
        async fn update_event(
            &self,
            request: tonic::Request<super::UpdateEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateEventResponse>,
            tonic::Status,
        >;
        async fn delete_event(
            &self,
            request: tonic::Request<super::DeleteEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEventResponse>,
            tonic::Status,
        >;
        async fn list_events(
            &self,
            request: tonic::Request<super::ListEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEventsResponse>,
            tonic::Status,
        >;
        async fn cancel_event(
            &self,
            request: tonic::Request<super::CancelEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelEventResponse>,
            tonic::Status,
        >;
        async fn create_booking(
            &self,
            request: tonic::Request<super::CreateBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBookingResponse>,
            tonic::Status,
        >;
        async fn get_booking(
            &self,
            request: tonic::Request<super::GetBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBookingResponse>,
            tonic::Status,
        >;
        async fn delete_booking(
            &self,
            request: tonic::Request<super::DeleteBookingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBookingResponse>,
            tonic::Status,
        >;
        async fn list_bookings(
            &self,
            request: tonic::Request<super::ListBookingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBookingsResponse>,
            tonic::Status,
        >;
        async fn create_closure(
            &self,
            request: tonic::Request<super::CreateClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateClosureResponse>,
            tonic::Status,
        >;
        async fn update_closure(
            &self,
            request: tonic::Request<super::UpdateClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateClosureResponse>,
            tonic::Status,
        >;
        async fn delete_closure(
            &self,
            request: tonic::Request<super::DeleteClosureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteClosureResponse>,
            tonic::Status,
        >;
        async fn list_closures(
            &self,
            request: tonic::Request<super::ListClosuresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListClosuresResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BookingServiceServer<T: BookingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BookingService> BookingServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BookingServiceServer<T>
    where
        T: BookingService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/booking.v1.BookingService/CreateEvent" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEventSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::CreateEventRequest>
                    for CreateEventSvc<T> {
                        type Response = super::CreateEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::create_event(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/GetEvent" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::GetEventRequest>
                    for GetEventSvc<T> {
                        type Response = super::GetEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::get_event(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/UpdateEvent" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEventSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::UpdateEventRequest>
                    for UpdateEventSvc<T> {
                        type Response = super::UpdateEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::update_event(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/DeleteEvent" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEventSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::DeleteEventRequest>
                    for DeleteEventSvc<T> {
                        type Response = super::DeleteEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::delete_event(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/ListEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListEventsSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::ListEventsRequest>
                    for ListEventsSvc<T> {
                        type Response = super::ListEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::list_events(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/CancelEvent" => {
                    #[allow(non_camel_case_types)]
                    struct CancelEventSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::CancelEventRequest>
                    for CancelEventSvc<T> {
                        type Response = super::CancelEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelEventRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::cancel_event(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/CreateBooking" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBookingSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::CreateBookingRequest>
                    for CreateBookingSvc<T> {
                        type Response = super::CreateBookingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBookingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::create_booking(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateBookingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/GetBooking" => {
                    #[allow(non_camel_case_types)]
                    struct GetBookingSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::GetBookingRequest>
                    for GetBookingSvc<T> {
                        type Response = super::GetBookingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBookingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::get_booking(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBookingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/DeleteBooking" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBookingSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::DeleteBookingRequest>
                    for DeleteBookingSvc<T> {
                        type Response = super::DeleteBookingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBookingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::delete_booking(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBookingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/ListBookings" => {
                    #[allow(non_camel_case_types)]
                    struct ListBookingsSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::ListBookingsRequest>
                    for ListBookingsSvc<T> {
                        type Response = super::ListBookingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBookingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::list_bookings(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListBookingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/CreateClosure" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClosureSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::CreateClosureRequest>
                    for CreateClosureSvc<T> {
                        type Response = super::CreateClosureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateClosureRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::create_closure(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateClosureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/UpdateClosure" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateClosureSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::UpdateClosureRequest>
                    for UpdateClosureSvc<T> {
                        type Response = super::UpdateClosureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateClosureRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::update_closure(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateClosureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/DeleteClosure" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteClosureSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::DeleteClosureRequest>
                    for DeleteClosureSvc<T> {
                        type Response = super::DeleteClosureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteClosureRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::delete_closure(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteClosureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/booking.v1.BookingService/ListClosures" => {
                    #[allow(non_camel_case_types)]
                    struct ListClosuresSvc<T: BookingService>(pub Arc<T>);
                    impl<
                        T: BookingService,
                    > tonic::server::UnaryService<super::ListClosuresRequest>
                    for ListClosuresSvc<T> {
                        type Response = super::ListClosuresResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListClosuresRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BookingService>::list_closures(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListClosuresSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BookingService> Clone for BookingServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: BookingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BookingService> tonic::server::NamedService for BookingServiceServer<T> {
        const NAME: &'static str = "booking.v1.BookingService";
    }
}
