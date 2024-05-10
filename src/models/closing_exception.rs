use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;
use protos::booking::v1::{TimeData};
use crate::models::event::{Event, EventWithSlots};
use crate::schema::{closing_exceptions};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = closing_exceptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClosingException {
    pub id: Uuid,
    pub event_id: Uuid,
    pub closing_from: NaiveDateTime,
    pub closing_to: NaiveDateTime,
    pub reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = closing_exceptions)]
pub struct NewClosingException<'a> {
    pub event_id: &'a Uuid,
    pub closing_from: &'a NaiveDateTime,
    pub closing_to: &'a NaiveDateTime,
    pub reason: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct ClosingExceptionWithEvent {
    pub closing_exception: ClosingException,
    pub event: EventWithSlots,
}

impl ClosingExceptionWithEvent {
    pub fn new(exception: ClosingException, event: EventWithSlots) -> Self {
        ClosingExceptionWithEvent {
            closing_exception: exception,
            event,
        }
    }
}

impl ClosingException {
    pub fn create(
        conn: &mut PgConnection,
        exception: NewClosingException,
    ) -> Result<ClosingExceptionWithEvent, diesel::result::Error> {
        match diesel::insert_into(closing_exceptions::table)
            .values(exception)
            .returning(ClosingException::as_returning())
            .get_result(conn)
        {
            Ok(exception) => {
                let event = Event::find_by_id(conn, exception.event_id).unwrap();
                Ok(ClosingExceptionWithEvent {
                    closing_exception: exception,
                    event,
                })
            },
            Err(e) => {
                log::error!("Failed to create booking: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Option<ClosingExceptionWithEvent> {
        let exception = closing_exceptions::dsl::closing_exceptions
            .select(ClosingException::as_select())
            .filter(closing_exceptions::dsl::id.eq(id))
            .first(conn)
            .ok();

        // TODO: Refactor this
        match exception {
            Some(e) => {
                let event = Event::find_by_id(conn, e.event_id);
                match event {
                    Some(event) => Some(ClosingExceptionWithEvent::new(e, event)),
                    None => None
                }
            },
            None => None
        }
    }
}

impl From<ClosingException> for protos::booking::v1::ClosingException {
    fn from(exception: ClosingException) -> Self {
        let mut proto_exception = protos::booking::v1::ClosingException::default();

        proto_exception.id = exception.id.to_string();
        proto_exception.event_id = exception.event_id.to_string();
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

impl From<ClosingExceptionWithEvent> for protos::booking::v1::ClosingException {
    fn from(exception_with_event: ClosingExceptionWithEvent) -> Self {
        let mut proto_exception = protos::booking::v1::ClosingException::from(exception_with_event.closing_exception);

        proto_exception.event = Some(exception_with_event.event.into());
        proto_exception
    }
}
