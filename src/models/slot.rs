use chrono::{NaiveDateTime};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel::data_types::{PgTime};
use uuid::Uuid;
use protos::booking::v1::{TimeData};
use diesel::prelude::*;
use diesel::sql_query;
use booking_ms::pg_time_to_string;
use crate::models::event::Event;
use crate::schema::event_slots;

#[derive(Queryable, Selectable, QueryableByName, PartialEq, Debug, Clone)]
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

    pub fn find_by_id(conn: &mut PgConnection, p_slot_id: Uuid) -> Option<Slot> {
        event_slots::dsl::event_slots
            .select(Slot::as_select())
            .filter(event_slots::dsl::id.eq(p_slot_id))
            .first(conn)
            .ok()
    }

    pub fn find_by_event_id(conn: &mut PgConnection, p_event_id: Uuid) -> Option<Vec<Slot>> {
        event_slots::dsl::event_slots
            .select(Slot::as_select())
            .filter(event_slots::dsl::event_id.eq(p_event_id))
            .load(conn)
            .ok()
    }

    pub fn find_active_by_event(conn: &mut PgConnection, event: &Event) -> Option<Vec<Slot>> {
        sql_query("
            SELECT * FROM event_slots es
            WHERE NOT EXISTS (
                SELECT 1
                FROM closures ce
                WHERE es.start_time BETWEEN CAST(ce.closing_from AS TIME) AND CAST(ce.closing_to AS TIME)
                    AND ce.organizer_key = $2
            )
            AND es.event_id = $1
            GROUP BY es.id
            ORDER BY es.start_time
        ")
            .bind::<diesel::sql_types::Uuid, _>(event.id)
            .bind::<diesel::sql_types::VarChar, _>(event.organizer_key.clone())
            .load::<Slot>(conn)
            .ok()
    }
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