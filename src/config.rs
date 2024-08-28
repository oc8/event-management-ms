use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub database_min_connections: u32,
    pub database_max_connections: u32,
    pub database_max_lifetime: u64,
    pub redis_hostname: String,
    pub redis_password: String,
    pub redis_tls: bool,
    pub enable_cache: bool,
    pub cache_ttl: u64,
    pub enable_ipv6: bool,
    pub port: u16,
    pub metrics_port: u16,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    pub ca_cert: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
            )
            .set_default("database_min_connections", 1).unwrap()
            .set_default("database_max_connections", 16).unwrap()
            .set_default("database_max_lifetime", 3600).unwrap()
            .set_default("redis_hostname", "").unwrap()
            .set_default("redis_password", "").unwrap()
            .set_default("redis_tls", false).unwrap()
            .set_default("cache_ttl", 60).unwrap()
            .set_default("enable_cache", false).unwrap()
            .set_default("enable_ipv6", false).unwrap()
            .set_default("port", 50051).unwrap()
            .set_default("metrics_port", 3000).unwrap()
            .set_default("tls_cert", None::<String>).unwrap()
            .set_default("tls_key", None::<String>).unwrap()
            .set_default("ca_cert", None::<String>).unwrap()
            .build()?;

        let cfg: Config = config.try_deserialize()?;

        log::debug!("Config: {:?}", cfg);

        Ok(cfg)
    }
}
