use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;
use protos::booking::v1::{TimeData};
use crate::schema::{closing_exceptions};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = closing_exceptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClosingException {
    pub id: Uuid,
    pub closing_from: NaiveDateTime,
    pub closing_to: NaiveDateTime,
    pub reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = closing_exceptions)]
pub struct NewClosingException<'a> {
    pub closing_from: &'a NaiveDateTime,
    pub closing_to: &'a NaiveDateTime,
    pub reason: Option<&'a str>,
}

impl ClosingException {
    pub fn create(
        conn: &mut PgConnection,
        exception: NewClosingException,
    ) -> Result<ClosingException, diesel::result::Error> {
        match diesel::insert_into(closing_exceptions::table)
            .values(exception)
            .returning(ClosingException::as_returning())
            .get_result(conn)
        {
            Ok(exception) => Ok(exception),
            Err(e) => {
                log::error!("Failed to create closing_exception: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, slot_id: Uuid) -> Option<ClosingException> {
        closing_exceptions::dsl::closing_exceptions
            .select(ClosingException::as_select())
            .filter(closing_exceptions::dsl::id.eq(slot_id))
            .first(conn)
            .ok()
    }
}

impl From<ClosingException> for protos::booking::v1::ClosingException {
    fn from(exception: ClosingException) -> Self {
        let mut proto_exception = protos::booking::v1::ClosingException::default();

        proto_exception.id = exception.id.to_string();
        proto_exception.closing_from = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(exception.closing_from, Utc).to_rfc3339()
        });

        proto_exception.closing_to = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(exception.closing_to, Utc).to_rfc3339()
        });
        proto_exception.reason = exception.reason.unwrap_or_default();
        proto_exception.created_at = exception.created_at.and_utc().timestamp();
        proto_exception.updated_at = exception.updated_at.and_utc().timestamp();

        proto_exception
    }
}
