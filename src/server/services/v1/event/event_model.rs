use crate::errors::ApiError;
use num_derive::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use crate::server::services::v1::slot::slot_model::Slot;
use crate::utils::filters::{EventFilters, Filters};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use event_protos::event::v1::{
    Cancellation, EventStatus as EventStatusProto, EventType as EventTypeProto, TimeData,
};
use serde::Serialize;
use sqlx::postgres::types::PgInterval;

/// Defines the supported event status.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, sqlx::Type, Copy, Clone, Serialize)]
#[sqlx(type_name = "event_status", rename_all = "lowercase")]
pub enum EventStatus {
    Unspecified,
    Active,
    Canceled,
    Closed,
    Full,
    Disabled,
}

impl EventStatus {
    fn as_proto(&self) -> EventStatusProto {
        match self {
            EventStatus::Unspecified => EventStatusProto::Unspecified,
            EventStatus::Active => EventStatusProto::Active,
            EventStatus::Canceled => EventStatusProto::Canceled,
            EventStatus::Closed => EventStatusProto::Closed,
            EventStatus::Full => EventStatusProto::Full,
            EventStatus::Disabled => EventStatusProto::Disabled,
        }
    }

    pub(crate) fn from_proto(proto: i32) -> Self {
        match proto {
            0 => EventStatus::Unspecified,
            1 => EventStatus::Active,
            2 => EventStatus::Canceled,
            3 => EventStatus::Closed,
            4 => EventStatus::Full,
            5 => EventStatus::Disabled,
            _ => EventStatus::Unspecified,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EventStatus::Unspecified => "Unspecified".to_string(),
            EventStatus::Active => "Active".to_string(),
            EventStatus::Canceled => "Canceled".to_string(),
            EventStatus::Closed => "Closed".to_string(),
            EventStatus::Full => "Full".to_string(),
            EventStatus::Disabled => "Disabled".to_string(),
        }
    }
}

/// Defines the supported event types.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, sqlx::Type, Copy, Clone, Serialize)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    Unspecified,
    Event,
    Task,
    Meeting,
}

impl EventType {
    fn as_proto(&self) -> EventTypeProto {
        match self {
            EventType::Unspecified => EventTypeProto::Unspecified,
            EventType::Event => EventTypeProto::Event,
            EventType::Task => EventTypeProto::Task,
            EventType::Meeting => EventTypeProto::Meeting,
        }
    }

    pub(crate) fn from_proto(proto: i32) -> Self {
        match proto {
            0 => EventType::Unspecified,
            1 => EventType::Event,
            2 => EventType::Task,
            3 => EventType::Meeting,
            _ => EventType::Unspecified,
        }
    }
}

/// Defines the full structure of an event.
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub status: EventStatus,
    pub event_type: EventType,
    pub slots: Option<Vec<Slot>>,
    pub overlap: bool,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub recurrence_rule: Option<String>,
    pub organizer_key: String,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub slot_capacity: Option<i32>,
    pub capacity: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Event {
    pub(crate) fn to_response(self, tz: Tz) -> event_protos::event::v1::Event {
        let mut proto_event = event_protos::event::v1::Event::default();

        let tz_offset = tz.offset_from_utc_datetime(&Utc::now().naive_utc());
        let start_datetime = DateTime::<Tz>::from_naive_utc_and_offset(self.start_time, tz_offset);
        let end_datetime = DateTime::<Tz>::from_naive_utc_and_offset(self.end_time, tz_offset);

        proto_event.id = self.id.to_string();
        proto_event.name = self.name;
        proto_event.set_status(self.status.as_proto());
        proto_event.set_event_type(self.event_type.as_proto());
        proto_event.slots = match self.slots {
            Some(slots) => slots.into_iter().map(|slot| slot.to_response_with_event_time(self.start_time, tz)).collect(),
            None => vec![],
        };
        proto_event.start = Some(TimeData {
            timezone: tz.to_string(),
            date_time: start_datetime.to_rfc3339(),
        });
        proto_event.end = Some(TimeData {
            timezone: tz.to_string(),
            date_time: end_datetime.to_rfc3339(),
        });
        proto_event.recurrence_rule = self.recurrence_rule.unwrap_or_default();
        proto_event.organizer_key = self.organizer_key;
        proto_event.cancellation = match self.canceled_at {
            Some(_) => Some(Cancellation {
                canceled_by: self.canceled_by.unwrap_or_default(),
                reason: self.canceled_reason.unwrap_or_default(),
                created_at: Some(TimeData {
                    timezone: tz.to_string(),
                    date_time: DateTime::<Tz>::from_naive_utc_and_offset(
                        self.canceled_at.unwrap(),
                        tz_offset,
                    )
                    .to_rfc3339(),
                }),
            }),
            None => None,
        };
        proto_event.slot_duration = match self.slot_duration {
            Some(interval) => interval.microseconds / 60_000_000,
            None => 0,
        };
        proto_event.capacity = self.capacity.unwrap_or_default();
        proto_event.created_at = self.created_at.and_utc().timestamp();
        proto_event.updated_at = self.updated_at.and_utc().timestamp();

        proto_event
    }
}

/// Defines the full database structure of an event.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct DbEvent {
    pub id: Uuid,
    pub name: String,
    pub status: EventStatus,
    pub event_type: EventType,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub recurrence_rule: Option<String>,
    pub organizer_key: String,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub slot_capacity: Option<i32>,
    pub capacity: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbEvent {
    pub fn into_event(self, slots: Option<Vec<Slot>>) -> Event {
        Event {
            id: self.id,
            name: self.name,
            status: self.status,
            event_type: self.event_type,
            slots,
            overlap: false,
            start_time: self.start_time,
            end_time: self.end_time,
            recurrence_rule: self.recurrence_rule,
            organizer_key: self.organizer_key,
            canceled_by: self.canceled_by,
            canceled_at: self.canceled_at,
            canceled_reason: self.canceled_reason,
            slot_duration: self.slot_duration,
            slot_capacity: self.slot_capacity,
            capacity: self.capacity,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Defines an event structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub(crate) struct EventInsert {
    pub name: String,
    pub status: EventStatus,
    pub event_type: EventType,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub recurrence_rule: Option<String>,
    pub organizer_key: String,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub slot_duration: Option<PgInterval>,
    pub capacity: Option<i32>,
    pub slot_capacity: Option<i32>,
}

/// Defines an event structure that can be updated in the database.
#[derive(Debug, PartialEq, Default)]
pub(crate) struct EventUpdate {
    pub name: Option<String>,
    pub status: Option<EventStatus>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub recurrence_rule: Option<String>,
    pub canceled_by: Option<String>,
    pub canceled_at: Option<NaiveDateTime>,
    pub canceled_reason: Option<String>,
    pub capacity: Option<i32>,
    pub slot_capacity: Option<i32>,
}

#[async_trait]
pub(crate) trait EventRepository: Send + Sync + 'static {
    /// Attempts to create a new event.
    ///
    /// # Parameters
    /// A struct containing the details of the event to be created.
    ///
    /// ## Success
    /// A struct containing the newly created event.
    ///
    /// ## Errors
    /// An error could occur if the event already exists, or a failure occurred with the
    /// database.
    async fn create_event(&mut self, event: &EventInsert) -> Result<Event, ApiError>;

    /// Attempts to retrieve an event by its id.
    ///
    /// # Parameters
    /// The id of the event to be retrieved.
    ///
    /// ## Success
    /// A struct containing the event
    ///
    /// ## Errors
    /// An error could occur if the event does not exist, or a failure occurred with the
    /// database.
    async fn get_event_by_id(&mut self, id: Uuid) -> Result<Event, ApiError>;

    /// Attempts to retrieve a list of events with filters.
    ///
    /// # Parameters
    /// A struct containing the filters to be applied.
    ///
    /// ## Success
    /// A vector containing the events that match the filter.
    ///
    /// ## Errors
    /// An error could occur if a failure occurred with the database.
    async fn get_events_with_filter(
        &mut self,
        filters: &Filters<EventFilters>,
    ) -> Result<Vec<Event>, ApiError>;

    /// Attempts to update an event by its id.
    ///
    /// # Parameters
    /// The id of the event to be updated.
    /// A struct containing the details of the event to be updated.
    ///
    /// ## Success
    /// A struct containing the updated event.
    ///
    /// ## Errors
    /// An error could occur if the event does not exist, or a failure occurred with the
    /// database.
    async fn update_event(&mut self, id: Uuid, event: &EventUpdate) -> Result<Event, ApiError>;

    /// Attempts to delete an event by its id.
    ///
    /// # Parameters
    /// The id of the event to be deleted.
    ///
    /// ## Success
    /// The number of rows affected by the delete operation.
    ///
    /// ## Errors
    /// An error could occur if the event does not exist, or a failure occurred with the
    /// database.
    async fn delete_event(&mut self, id: Uuid) -> Result<u64, ApiError>;
}

#[async_trait]
pub(crate) trait EventActions: Send + Sync + 'static {
    /// Generates a list of available dates for the event.
    ///
    /// # Parameters
    /// The start date to begin generating available dates.
    /// The limit of dates to generate.
    ///
    /// ## Success
    /// A list of available dates for the event.
    fn get_available_dates(
        &self,
        start: NaiveDateTime,
        limit: u16,
    ) -> Result<Vec<NaiveDate>, ApiError>;
}
