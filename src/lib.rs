use std::env;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use ::log::{error, info};
use chrono::{Datelike, DateTime, MappedLocalTime, NaiveDateTime, NaiveTime, Timelike, TimeZone};
use rrule::Tz;

pub mod server;
pub mod database;
pub mod errors;
pub mod utils;

pub fn init_service_logging() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .parse_env("RUST_LOG")
        .init();
}

pub fn report_error<E: 'static>(err: &E)
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

pub fn add_time_to_datetime(datetime: NaiveDateTime, time: NaiveTime) -> NaiveDateTime {
    let date_part = datetime.date();
    let time_part = time;

    NaiveDateTime::new(date_part, time_part)
}

pub fn naive_datetime_to_rrule_datetime(datetime: NaiveDateTime) -> MappedLocalTime<DateTime<Tz>> {
    Tz::UTC.with_ymd_and_hms(datetime.year(), datetime.month(), datetime.day(), datetime.hour(), datetime.minute(), datetime.second())
}

// Remove seconds and milliseconds from start and end time to keep a consistent format
pub fn truncate_to_minute(datetime: &NaiveDateTime) -> NaiveDateTime {
    datetime
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}