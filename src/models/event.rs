use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use diesel::data_types::PgInterval;
use uuid::Uuid;
use protos::booking::v1::{Cancellation, EventStatus, EventType, TimeData};

use crate::schema::events;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub event_type: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub recurrence_rule: Option<String>,
    pub timezone: String,
    pub organizer_key: String,
    pub max_guests: Option<i32>,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub max_persons_per_slot: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub status: &'a str,
    pub event_type: &'a str,
    pub start_time: &'a NaiveDateTime,
    pub end_time: &'a NaiveDateTime,
    pub recurrence_rule: Option<&'a str>,
    pub timezone: &'a str,
    pub organizer_key: &'a str,
    pub max_guests: Option<&'a i32>,
    pub canceled_by: Option<&'a str>,
    pub canceled_at: Option<&'a NaiveDateTime>,
    pub canceled_reason: Option<&'a str>,
    pub slot_duration: Option<&'a PgInterval>,
    pub max_persons_per_slot: Option<&'a i32>,
}

impl Event {
    pub fn create(
        conn: &mut PgConnection,
        event: NewEvent,
    ) -> Result<Event, diesel::result::Error> {
        match diesel::insert_into(events::table)
            .values(event)
            .returning(Event::as_returning())
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => {
                log::error!("Failed to create user: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Option<Event> {
        events::dsl::events
            .select(Event::as_select())
            .filter(events::dsl::id.eq(id))
            .first(conn)
            .ok()
    }
}

impl From<Event> for protos::booking::v1::Event {
    fn from(event: Event) -> Self {
        let mut proto_event = protos::booking::v1::Event::default();

        proto_event.id = event.id.to_string();
        proto_event.name = event.name;
        proto_event.set_status(EventStatus::from_str_name(&event.status).unwrap_or(EventStatus::Unspecified));
        proto_event.set_event_type(EventType::from_str_name(&event.event_type).unwrap_or(EventType::Event));
        proto_event.start = Some(TimeData {
            timezone: event.timezone.clone(),
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(event.start_time, Utc).to_rfc3339()
        });
        proto_event.end = Some(TimeData {
            timezone: event.timezone.clone(),
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(event.end_time, Utc).to_rfc3339()
        });
        proto_event.recurrence_rule = event.recurrence_rule.unwrap_or_default();
        proto_event.organizer_key = event.organizer_key;
        proto_event.max_guests = event.max_guests.unwrap_or_default();
        proto_event.cancellation = match event.canceled_at {
            Some(_) => Some(Cancellation {
                canceled_by: event.canceled_by.unwrap_or_default(),
                reason: event.canceled_reason.unwrap_or_default(),
                created_at: Some(TimeData {
                    timezone: event.timezone.clone(),
                    date_time: event.canceled_at.unwrap().to_string()
                })
            }),
            None => None
        };
        proto_event.slot_duration = match event.slot_duration {
            Some(interval) => interval.microseconds / 60_000_000,
            None => 0
        };
        proto_event.created_at = event.created_at.and_utc().timestamp();
        proto_event.updated_at = event.updated_at.and_utc().timestamp();

        proto_event
    }
}