use std::env;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use rand::Rng;
use redis::{Commands, RedisResult};
use ::log::{error, info};
use chrono::NaiveDateTime;
use protos::booking::v1::Filters as FiltersProto;

pub fn init_service_logging() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .parse_env("RUST_LOG")
        .init();
}

pub fn env_var(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

pub fn report_error<E: 'static>(err: E)
    where
        E: std::error::Error,
        E: Send + Sync,
{
    let mut stack = String::from("\n");
    if let Some(cause) = err.source() {
        for (i, e) in std::iter::successors(Some(cause), |e| e.source()).enumerate() {
            stack.push_str(&format!("   {}: {}\n", i, e));
        }
    }
    error!("[ERROR] {}\nCaused by: {}", err, stack);
}

pub fn create_socket_addr(port: u16) -> SocketAddr {
    let is_ipv6 = env::var("ENABLE_IPV6").unwrap_or_default().parse::<bool>().unwrap_or(false);

    if is_ipv6 {
        info!("Using IPv6");
        SocketAddr::from(SocketAddrV6::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
            port,
            0,
            0,
        ))
    } else {
        info!("Using IPv4");
        SocketAddr::from(([0, 0, 0, 0], port))
    }
}

pub fn format_datetime(datetime: NaiveDateTime) -> String {
    datetime.format("%Y%m%dT%H%M%SZ").to_string()
}

pub struct Filters {
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub organizer_key: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Filters {
    pub fn new(from: Option<NaiveDateTime>, to: Option<NaiveDateTime>, organizer_key: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Self {
        Filters {
            from,
            to,
            organizer_key,
            limit,
            offset,
        }
    }
}

impl From<Option<FiltersProto>> for Filters {
    fn from(proto: Option<FiltersProto>) -> Self {
        let proto = proto.unwrap();

        let from = if proto.from.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(&proto.from, "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let to = if proto.to.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(&proto.to, "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let limit = if proto.limit == 0 {
            Some(50)
        } else {
            Some(proto.limit)
        };

        Filters {
            from,
            to,
            organizer_key: Some(proto.organizer_key),
            limit,
            offset: Some(proto.offset),
        }
    }
}
