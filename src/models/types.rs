use std::io::Write;
use diesel::{FromSqlRow, serialize, deserialize, SqlType};
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};

#[derive(SqlType)]
#[diesel(postgres_type(name = "event_status"))]
pub struct EventStatusType;

#[derive(Debug, PartialEq, FromSqlRow, Eq)]
#[diesel(sql_type = EventStatusType)]
pub enum EventStatus {
    Unspecified,
    Active,
    Canceled,
    Full,
}

impl ToSql<EventStatusType, Pg> for EventStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            EventStatus::Unspecified => out.write_all(b"unspecified")?,
            EventStatus::Active => out.write_all(b"active")?,
            EventStatus::Canceled => out.write_all(b"canceled")?,
            EventStatus::Full => out.write_all(b"full")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<EventStatusType, Pg> for EventStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"unspecified" => Ok(EventStatus::Unspecified),
            b"active" => Ok(EventStatus::Active),
            b"canceled" => Ok(EventStatus::Canceled),
            b"full" => Ok(EventStatus::Full),
            _ => Err("Unknown Event Status".into()),
        }
    }
}
