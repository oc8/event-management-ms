use std::sync::Arc;

use axum::{routing::get, Router};
use autometrics::prometheus_exporter;
use dotenvy::dotenv;
use event_ms::{create_socket_addr, database, init_service_logging, Config};
use event_ms::database::CacheClient;
use event_ms::server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_service_logging();
    prometheus_exporter::init();

    let cfg = Config::from_env().expect("Failed to load configuration");

    // Set up the database connection
    let pool = database::connect(&cfg)
        .await
        .expect("Couldn't connect to the database");

    // Set up the Redis connection
    let uri_scheme = match cfg.redis_tls {
        true => "rediss",
        false => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, cfg.redis_password, cfg.redis_hostname);
    log::info!("Connecting to Redis at {}", redis_conn_url);
    let r_client = database::connect_redis(redis_conn_url)
        .expect("Couldn't connect to Redis");

    // Start the gRPC server
    let cache_client = CacheClient::new(r_client, cfg.cache_ttl);

    let _server = start_server(
        cfg.clone(),
        Arc::new(pool),
        cache_client
    );

    let app = Router::new().route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let metrics_addr = create_socket_addr(cfg.metrics_port, cfg.enable_ipv6);
    let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
    log::info!("Metrics server listening on port {}", cfg.metrics_port);
    axum::serve(listener, app).await.unwrap();

    // TODO: fix ghost process on mac ctrl+c
    Ok(())
}