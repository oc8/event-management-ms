use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::str::FromStr;
use ::log::{error, info};
use chrono::{Datelike, DateTime, MappedLocalTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike, TimeZone, Utc};
use rrule::Tz;
use tonic::metadata::MetadataMap;

pub mod server;
pub mod database;
pub mod errors;
pub mod utils;
pub mod config;
pub use config::Config;

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

pub fn create_socket_addr(port: u16, enable_ipv6: bool) -> SocketAddr {
    if enable_ipv6 {
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

/// Remove seconds and milliseconds from start and end time to keep a consistent format
pub fn truncate_to_minute(datetime: &NaiveDateTime) -> NaiveDateTime {
    datetime
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}

/// Add the offset of the given timezone to the given time
pub fn add_time_offset(time: NaiveTime, tz: chrono_tz::Tz) -> NaiveTime {
    let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let naive_datetime = NaiveDateTime::new(date, time);

    let offset = tz.offset_from_utc_datetime(&Utc::now().naive_utc());

    DateTime::<chrono_tz::Tz>::from_naive_utc_and_offset(naive_datetime, offset).time()
}

/// Get the timezone from the metadata or return UTC if not present or invalid
fn get_meta_timezone(meta: &MetadataMap) -> chrono_tz::Tz {
    match meta.get("timezone") {
        Some(tz) => chrono_tz::Tz::from_str(tz.to_str().unwrap()).unwrap_or(chrono_tz::Tz::UTC),
        None => chrono_tz::Tz::UTC
    }
}