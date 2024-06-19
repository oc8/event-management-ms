// fn generate_cache_key(&self, method_name: &str, request: &impl Serialize) -> String {
//     format!("{}:{}", method_name, serde_json::to_string(request).unwrap())
// }
//
// async fn get_cache<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Status> {
//     let cache = self.cache.read().await;
//     let mut conn = cache.get_connection().map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     let data: Option<Vec<u8>> = conn.get(key).map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     if let Some(data) = data {
//         let result = serde_json::from_slice(&data).map_err(|e| {
//             report_error(&e);
//             format_error(ApiError {
//                 grpc_code: tonic::Code::Internal,
//                 code: "deserialization_error",
//                 message: "Failed to deserialize cached data",
//             })
//         })?;
//
//         log::debug!("Cache hit for key: {}", key);
//
//         Ok(Some(result))
//     } else {
//
//         log::debug!("Cache miss for key: {}", key);
//
//         Ok(None)
//     }
// }
//
// async fn set_cache<T: Serialize>(&self, key: &str, value: &T) -> Result<(), Status> {
//     let cache_ttl = std::env::var("CACHE_TTL")
//         .unwrap_or_else(|_| "60".to_string())
//         .parse::<u64>()
//         .unwrap();
//
//     let cache = self.cache.read().await;
//     let mut conn = cache.get_connection().map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     let data = serde_json::to_vec(value).map_err(|e| {
//         report_error(&e);
//         format_error(ApiError {
//             grpc_code: tonic::Code::Internal,
//             code: "serialization_error",
//             message: "Failed to serialize response",
//         })
//     })?;
//
//     conn.set_ex(key, data, cache_ttl).map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     log::debug!("Cache set for key: {}", key);
//
//     Ok(())
// }
//
// async fn invalid_cache(&self, method_name: &str, request: &impl Serialize) -> Result<(), Status> {
//     let cache_key = self.generate_cache_key(method_name, request);
//     let cache = self.cache.read().await;
//     let mut conn = cache.get_connection().map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     conn.del(cache_key).map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     Ok(())
// }
//
// async fn invalidate_related_cache_keys(&self, organizer_key: String) -> Result<(), Status> {
//     let cache = self.cache.read().await;
//     let mut conn = cache.get_connection().map_err(|e| {
//         report_error(&e);
//         format_error(errors::REDIS_CONNECTION_FAILURE)
//     })?;
//
//     let keys_to_invalidate = vec![
//         "list_*:{\"filters\":{*\"organizerKey\":\"".to_string() + &organizer_key + "\"*}*",
//     ];
//
//     log::debug!("Invalidating cache keys: {:?}", keys_to_invalidate);
//
//     for key_pattern in keys_to_invalidate {
//         let keys: Vec<String> = conn.keys(key_pattern).map_err(|e| {
//             report_error(&e);
//             format_error(errors::REDIS_CONNECTION_FAILURE)
//         })?;
//         for key in keys {
//             conn.del(&key).map_err(|e| {
//                 report_error(&e);
//                 format_error(errors::REDIS_CONNECTION_FAILURE)
//             })?;
//         }
//     }
//
//     Ok(())
// }
//
// async fn handle_cache<T, F>(
//     &self,
//     method_name: &str,
//     request: &impl Serialize,
//     call: F,
// ) -> Result<Response<T>, Status>
//     where
//         T: DeserializeOwned + Serialize,
//         F: FnOnce() -> Result<Response<T>, Status> + Send,
// {
//     let cache_key = self.generate_cache_key(method_name, request);
//     if let Some(cached_response) = self.get_cache::<T>(&cache_key).await? {
//         return Ok(Response::new(cached_response));
//     }
//
//     let response = call()?;
//
//     self.set_cache(&cache_key, response.get_ref()).await?;
//     Ok(response)
// }