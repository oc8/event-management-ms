use std::env;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tonic::Status;

pub type PgPooledConnection = sqlx::pool::PoolConnection<Postgres>;
pub type PgPool = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    log::info!("Connecting to database: {}", database_url);

    let pool = PgPoolOptions::new()
        .min_connections(
            env::var("DATABASE_MIN_CONNECTIONS")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(0),
        )
        .max_connections(
            env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(16),
        )
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    pool.acquire().await.map_err(|_| Status::internal("Failed to acquire connection"))
}
