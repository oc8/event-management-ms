use serde_json::Value as JsonValue;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use diesel::data_types::PgInterval;
use uuid::Uuid;

use crate::schema::events;

#[derive(Queryable, Selectable)]
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
    pub location_type: String,
    pub location: Option<String>,
    pub location_infos: Option<JsonValue>,
    pub organizer_name: Option<String>,
    pub organizer_email: Option<String>,
    pub max_guests: Option<i32>,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub max_persons_per_slot: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub status: &'a String,
    pub event_type: &'a String,
    pub start_time: &'a NaiveDateTime,
    pub end_time: &'a NaiveDateTime,
    pub recurrence_rule: Option<&'a str>,
    pub timezone: &'a str,
    pub location_type: &'a String,
    pub location: Option<&'a str>,
    pub location_infos: Option<&'a JsonValue>,
    pub organizer_name: Option<&'a str>,
    pub organizer_email: Option<&'a str>,
    pub max_guests: Option<i32>,
    pub canceled_by: Option<&'a str>,
    pub canceled_at: Option<&'a NaiveDateTime>,
    pub canceled_reason: Option<&'a str>,
    pub slot_duration: Option<&'a PgInterval>,
    pub max_persons_per_slot: Option<i32>,
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