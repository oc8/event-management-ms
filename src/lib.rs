use std::env;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use rand::Rng;
use redis::{Commands, RedisResult};
use ::log::{error, info};
use chrono::{Duration, NaiveDateTime, NaiveTime};
use diesel::data_types::PgTime;
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

pub fn microseconds_to_naive_time(microseconds: i64) -> NaiveTime {
    let hours = microseconds / 3_600_000_000;
    let remaining_microseconds = microseconds % 3_600_000_000;
    let minutes = remaining_microseconds / 60_000_000;
    let remaining_microseconds = remaining_microseconds % 60_000_000;
    let seconds = remaining_microseconds / 1_000_000;
    let remaining_microseconds = remaining_microseconds % 1_000_000;
    let nanos = remaining_microseconds * 1000;

    NaiveTime::from_hms_micro_opt(hours as u32, minutes as u32, seconds as u32, nanos as u32)
        .expect("Failed to convert microseconds to NaiveTime")
}

pub fn add_time_to_datetime(datetime: NaiveDateTime, time: NaiveTime) -> NaiveDateTime {
    let date_part = datetime.date();
    let time_part = time;

    NaiveDateTime::new(date_part, time_part)
}

pub fn pg_time_to_string(time: PgTime) -> String {
    let duration = Duration::microseconds(time.0);

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;

    format!("{:02}:{:02}", hours, minutes)
}