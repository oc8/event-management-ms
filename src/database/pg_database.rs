use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tonic::Status;

pub type PgPooledConnection = sqlx::pool::PoolConnection<Postgres>;
pub type PgPool = Pool<Postgres>;

pub async fn connect(cfg: &crate::Config) -> Result<PgPool, sqlx::Error> {
    log::info!("Connecting to database: {}", cfg.database_url);

    let pool = PgPoolOptions::new()
        .min_connections(cfg.database_min_connections)
        .max_connections(cfg.database_max_connections)
        .max_lifetime(Some(Duration::from_secs(cfg.database_max_lifetime)))
        .connect(&cfg.database_url)
        .await?;

    Ok(pool)
}

pub async fn get_connection(pool: &PgPool) -> Result<PgPooledConnection, Status> {
    pool.acquire().await.map_err(|_| Status::internal("Failed to acquire connection"))
}
