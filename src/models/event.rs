use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper, QueryResult, Connection, QueryableByName};
use diesel::data_types::{PgInterval};
use rrule::RRuleSet;
use uuid::Uuid;
use booking_ms::format_datetime;
use protos::booking::v1::{Cancellation, EventStatus, EventType, TimeData};
use crate::models::slot::{Slot};
use crate::models::filters::{Filters};

use crate::schema::{events};

#[derive(Queryable, Selectable, QueryableByName, PartialEq, Debug, Clone)]
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
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub max_persons: Option<i32>,
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
    pub canceled_by: Option<&'a str>,
    pub canceled_at: Option<&'a NaiveDateTime>,
    pub canceled_reason: Option<&'a str>,
    pub slot_duration: Option<&'a PgInterval>,
    pub max_persons: Option<&'a i32>,
    pub max_persons_per_slot: Option<&'a i32>,
}

#[derive(Debug, Clone)]
pub struct EventWithSlots {
    pub event: Event,
    pub slots: Vec<Slot>,
}

impl EventWithSlots {
    pub fn new(event: Event, slots: Vec<Slot>) -> Self {
        EventWithSlots { event, slots }
    }
}

impl Event {
    pub fn create(
        conn: &mut PgConnection,
        event: NewEvent,
    ) -> Result<EventWithSlots, diesel::result::Error> {
        match diesel::insert_into(events::table)
            .values(event)
            .returning(Event::as_returning())
            .get_result(conn)
        {
            Ok(e) => {
                if e.event_type == EventType::as_str_name(&EventType::Meeting) {
                    let slots = Self::generate_time_slots(conn, e.clone())?;
                    return Ok(EventWithSlots::new(e, slots));
                }
                Ok(EventWithSlots::new(e, vec![]))
            },
            Err(e) => {
                log::error!("Failed to create event: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Option<EventWithSlots> {
        let event = events::dsl::events
            .select(Event::as_select())
            .filter(events::dsl::id.eq(id))
            .first(conn)
            .ok();

        match event {
            Some(e) => {
                let slots = Slot::find_by_event_id(conn, e.id)
                    .unwrap_or_else(|| vec![]);
                Some(EventWithSlots::new(e, slots))
            },
            None => None
        }
    }

    pub fn find_active_events(conn: &mut PgConnection, filters: Filters) -> Vec<EventWithSlots> {
        let mut query = events::table
            .select(Event::as_select())
            .order(events::start_time.asc())
            .into_boxed();

        // let events = utils::apply_filters(query.into_boxed(), &filters)
        //     .load::<Event>(conn)
        //     .unwrap_or_else(|_| vec![]);

        if let Some(from) = filters.from {
            query = query.filter(events::start_time.ge(from));
        }
        if let Some(to) = filters.to {
            query = query.filter(events::start_time.le(to));
        }
        if let Some(organizer_key) = &filters.organizer_key {
            query = query.filter(events::organizer_key.eq(organizer_key));
        }
        if let Some(limit) = filters.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = filters.offset {
            query = query.offset(offset);
        }

        let events = query
            .load::<Event>(conn)
            .unwrap_or_else(|_| vec![]);

        events
            .into_iter()
            .filter_map(|event| {
                // Some(EventWithSlots::new(event, vec![]));
                let slots = Slot::find_by_event_id(conn, event.id)
                    .unwrap_or_else(|| vec![]);
                if slots.is_empty() {
                    None
                } else {
                    Some(EventWithSlots::new(event, slots))
                }
            })
            .collect()
    }

    // TODO: Prevent double insertion of slots
    pub fn generate_time_slots(conn: &mut PgConnection, event: Event) -> QueryResult<Vec<Slot>> {
        conn.transaction(|pg_conn| {
            diesel::sql_query("
                WITH RECURSIVE slot_times AS (
                    SELECT
                        $1::TIMESTAMP AS slot_start_time,
                        $1::TIMESTAMP + INTERVAL '1 minute' * $3 AS slot_end_time
                    UNION ALL
                    SELECT
                        slot_end_time,
                        slot_end_time + INTERVAL '1 minute' * $3
                    FROM
                        slot_times
                    WHERE
                        slot_end_time < $2
                )
                INSERT INTO event_slots (event_id, start_time, end_time, max_persons)
                SELECT
                    $4,
                    slot_start_time,
                    slot_end_time,
                    $5
                FROM
                    slot_times;"
            )
                .bind::<diesel::sql_types::Timestamp, _>(event.start_time)
                .bind::<diesel::sql_types::Timestamp, _>(event.end_time)
                .bind::<diesel::sql_types::Integer, _>((event.slot_duration.unwrap().microseconds / 60_000_000) as i32)
                .bind::<diesel::sql_types::Uuid, _>(event.id)
                .bind::<diesel::sql_types::Integer, _>(event.max_persons_per_slot.unwrap_or(1))
                .execute(pg_conn)
        })
        .expect("Failed to generate time slots");

        // TODO: find a way to return the slots without querying the database again
        let slots = Slot::find_by_event_id(conn, event.id)
            .unwrap_or_else(|| vec![]);

        Ok(slots)
    }

    pub fn get_available_dates(&self, limit: u16) -> Vec<NaiveDate> {
        if let Some(recurrence_rule) = &self.recurrence_rule {
            let recurrence_rule = format!("DTSTART:{}\nRRULE:{}", format_datetime(self.start_time), recurrence_rule);
            let recurrence = recurrence_rule.parse::<RRuleSet>();

            match recurrence {
                Ok(recurrence) => {
                    recurrence.all(limit).dates
                        .into_iter()
                        .map(|date| date.naive_utc().date())
                        .collect()
                },
                Err(e) => {
                    log::error!("Failed to parse recurrence rule: {}", e);
                    vec![]
                }
            }
        } else {
            vec![]
        }
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
        proto_event.max_persons = event.max_persons.unwrap_or_default();
        proto_event.created_at = event.created_at.and_utc().timestamp();
        proto_event.updated_at = event.updated_at.and_utc().timestamp();

        proto_event
    }
}

impl From<EventWithSlots> for protos::booking::v1::Event {
    fn from(event_with_slots: EventWithSlots) -> Self {
        let mut proto_event = protos::booking::v1::Event::from(event_with_slots.event);

        let slots = event_with_slots.slots.into_iter().map(|slot| {
            protos::booking::v1::Slot::from(slot)
        });

        proto_event.slots = slots.collect();

        proto_event
    }
}
