use chrono::{Duration, NaiveDateTime};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel::data_types::{PgTime};
use uuid::Uuid;
use protos::booking::v1::{TimeData};

use crate::schema::event_slots;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = event_slots)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Slot {
    pub id: Uuid,
    pub event_id: Uuid,
    pub start_time: PgTime,
    pub end_time: PgTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = event_slots)]
pub struct NewSlot<'a> {
    pub event_id: &'a Uuid,
    pub start_time: &'a PgTime,
    pub end_time: &'a PgTime,
}

impl Slot {
    pub fn create(
        conn: &mut PgConnection,
        slot: NewSlot,
    ) -> Result<Slot, diesel::result::Error> {
        match diesel::insert_into(event_slots::table)
            .values(slot)
            .returning(Slot::as_returning())
            .get_result(conn)
        {
            Ok(slot) => Ok(slot),
            Err(e) => {
                log::error!("Failed to create slot: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Option<Slot> {
        event_slots::dsl::event_slots
            .select(Slot::as_select())
            .filter(event_slots::dsl::id.eq(id))
            .first(conn)
            .ok()
    }

    pub fn find_by_event_id(conn: &mut PgConnection, event_id: Uuid) -> Option<Vec<Slot>> {
        event_slots::dsl::event_slots
            .select(Slot::as_select())
            .filter(event_slots::dsl::event_id.eq(event_id))
            .load(conn)
            .ok()
    }
}

fn pg_time_to_string(time: PgTime) -> String {
    let duration = Duration::microseconds(time.0);

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;

    format!("{:02}:{:02}", hours, minutes)
}

impl From<Slot> for protos::booking::v1::Slot {
    fn from(slot: Slot) -> Self {
        let mut proto_slot = protos::booking::v1::Slot::default();

        let start = pg_time_to_string(slot.start_time);
        let end = pg_time_to_string(slot.end_time);

        proto_slot.id = slot.id.to_string();
        proto_slot.event_id = slot.event_id.to_string();
        proto_slot.start = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: start,
        });
        proto_slot.end = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: end,
        });
        proto_slot.max_guests = 0; // TODO: Get max guests from event
        proto_slot.created_at = slot.created_at.and_utc().timestamp();
        proto_slot.updated_at = slot.updated_at.and_utc().timestamp();

        proto_slot
    }
}