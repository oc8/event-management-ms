use crate::errors::ApiError;
use uuid::Uuid;

use crate::add_time_offset;
use crate::server::services::v1::event::event_model::{DbEvent, Event};
use async_trait::async_trait;
use chrono::{NaiveDateTime, NaiveTime};
use chrono_tz::Tz;
use event_protos::event::v1::{SlotStatus, TimeData};

/// Defines the full structure of a slot.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct DbSlot {
    pub id: Uuid,
    pub event_id: Uuid,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub capacity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbSlot {
    pub fn into_slot(self, status: SlotStatus, event: Option<Event>) -> Slot {
        Slot {
            id: self.id,
            event_id: self.event_id,
            status,
            event,
            start_time: self.start_time,
            end_time: self.end_time,
            capacity: self.capacity,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Defines the full structure of a slot.
#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub id: Uuid,
    pub event_id: Uuid,
    pub status: SlotStatus,
    pub event: Option<Event>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub capacity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Slot {
    pub(crate) fn to_response(self, tz: Tz) -> event_protos::event::v1::Slot {
        let mut proto_slot = event_protos::event::v1::Slot::default();

        proto_slot.id = self.id.to_string();
        proto_slot.event_id = self.event_id.to_string();
        proto_slot.set_status(self.status);
        proto_slot.start = Some(TimeData {
            timezone: tz.to_string(),
            date_time: add_time_offset(self.start_time, tz).to_string(),
        });
        proto_slot.end = Some(TimeData {
            timezone: tz.to_string(),
            date_time: add_time_offset(self.end_time, tz).to_string(),
        });
        proto_slot.capacity = self.capacity;
        proto_slot.created_at = self.created_at.and_utc().timestamp();
        proto_slot.updated_at = self.updated_at.and_utc().timestamp();

        proto_slot
    }
}

/// Defines a slot structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub(crate) struct SlotInsert {
    pub event_id: Uuid,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub capacity: i32,
}

#[async_trait]
pub(crate) trait SlotRepository: Send + Sync + 'static {
    /// Attempts to retrieve a slot by its id.
    ///
    /// # Parameters
    /// A struct containing the details of the slot to be created.
    ///
    /// ## Success
    /// A struct containing the newly created slot.
    ///
    /// ## Errors
    /// An error could occur if the event already has slots, or a failure occurred with the
    /// database.
    async fn get_slot_by_id(&mut self, id: Uuid) -> Result<Slot, ApiError>;

    /// Attempts to retrieve all slots for an event.
    ///
    /// # Parameters
    /// The id of the event to retrieve slots for.
    ///
    /// ## Success
    /// A vector containing all slots for the event.
    ///
    /// ## Errors
    /// An error could occur if the event does not exist, or a failure occurred with the
    /// database.
    async fn find_by_event_id(&mut self, event_id: Uuid) -> Result<Vec<Slot>, ApiError>;

    /// Generates slots for an event.
    ///
    /// # Parameters
    /// The event to generate slots for.
    ///
    /// ## Success
    /// A list of slots for the event.
    async fn generate_event_slots(&mut self, event: &DbEvent) -> Result<Vec<Slot>, ApiError>;
}
