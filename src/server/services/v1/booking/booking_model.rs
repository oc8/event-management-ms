use crate::errors::ApiError;
use uuid::Uuid;

use crate::server::services::v1::slot::slot_model::Slot;
use crate::utils::filters::{BookingFilters, Filters};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use event_protos::event::v1::TimeData;

/// Defines the full structure of a booking.
#[derive(Debug, PartialEq)]
pub struct Booking {
    pub id: Uuid,
    pub slot_id: Uuid,
    pub slot: Option<Slot>,
    pub booking_holder_key: String,
    pub organizer_key: String,
    pub date_time: NaiveDateTime,
    pub persons: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Booking {
    pub(crate) fn to_response(self, tz: Tz) -> event_protos::event::v1::Booking {
        let mut proto_booking = event_protos::event::v1::Booking::default();

        let offset = tz.offset_from_utc_datetime(&Utc::now().naive_utc());

        proto_booking.id = self.id.to_string();
        proto_booking.slot_id = self.slot_id.to_string();
        proto_booking.booking_holder_key = self.booking_holder_key;
        proto_booking.date_time = Some(TimeData {
            timezone: tz.to_string(),
            date_time: DateTime::<Tz>::from_naive_utc_and_offset(self.date_time, offset)
                .to_rfc3339(),
        });
        proto_booking.slot = self.slot.map(|s| s.to_response(Tz::UTC));
        proto_booking.persons = self.persons;
        proto_booking.created_at = self.created_at.and_utc().timestamp();
        proto_booking.updated_at = self.updated_at.and_utc().timestamp();

        proto_booking
    }
}

/// Defines the full database structure of a booking.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct DbBooking {
    pub id: Uuid,
    pub booking_holder_key: String,
    pub organizer_key: String,
    pub slot_id: Uuid,
    pub date_time: NaiveDateTime,
    pub persons: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbBooking {
    pub fn into_booking(self, slot: Option<Slot>) -> Booking {
        Booking {
            id: self.id,
            slot_id: self.slot_id,
            slot,
            booking_holder_key: self.booking_holder_key,
            organizer_key: self.organizer_key,
            date_time: self.date_time,
            persons: self.persons,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Defines a booking structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub(crate) struct BookingInsert {
    pub slot_id: Uuid,
    pub booking_holder_key: String,
    pub organizer_key: String,
    pub date_time: NaiveDateTime,
    pub persons: i32,
}

#[async_trait]
pub(crate) trait BookingRepository: Send + Sync + 'static {
    /// Attempts to create a new booking.
    ///
    /// # Parameters
    /// A struct containing the details of the booking to be created.
    ///
    /// ## Success
    /// A struct containing the newly created booking.
    ///
    /// ## Errors
    /// An error could occur if the booking already exists, or a failure occurred with the
    /// database.
    async fn create_booking(&mut self, booking: &BookingInsert) -> Result<Booking, ApiError>;

    /// Attempts to find a booking by their id.
    ///
    /// # Parameters
    /// The id of the booking to be found.
    ///
    /// ## Success
    /// A struct containing the booking that was found.
    ///
    /// ## Errors
    /// If the booking does not exist, or a failure occurred with the database.
    async fn get_booking_by_id(&mut self, slot_id: Uuid) -> Result<Booking, ApiError>;

    /// Attempts to retrieve a list of bookings with filters.
    ///
    /// # Parameters
    /// A struct containing the filters to be applied.
    ///
    /// ## Success
    /// A vector containing the bookings that match the filter.
    ///
    /// ## Errors
    /// An error could occur if a failure occurred with the database.
    async fn get_bookings_with_filters(
        &mut self,
        filters: &Filters<BookingFilters>,
    ) -> Result<Vec<Booking>, ApiError>;

    /// Attempts to delete a booking by their id.
    ///
    /// # Parameters
    /// The id of the booking to be deleted.
    ///
    /// ## Success
    /// The number of rows affected by the delete operation.
    ///
    /// ## Errors
    /// If the booking does not exist, or a failure occurred with the database.
    async fn delete_booking(&mut self, slot_id: Uuid) -> Result<usize, ApiError>;

    /// Attempts to retrieve an existing booking by the booking holder and datetime.
    ///
    /// # Parameters
    /// The id of the slot to retrieve the booking.
    /// The booking holder of the booking.
    /// The datetime of the booking.
    ///
    /// ## Success
    /// The booking that was found.
    ///
    /// ## Errors
    /// If the booking does not exist, or a failure occurred with the database.
    async fn get_booking_holder_booking(
        &mut self,
        slot_id: Uuid,
        booking_holder: String,
        date_time: NaiveDateTime,
    ) -> Result<Booking, ApiError>;

    /// Attempts to sum the number of persons by datetime.
    ///
    /// # Parameters
    /// The id of the slot to retrieve the bookings.
    /// The datetime of the booking.
    ///
    /// ## Success
    /// The number of persons, 0 if no bookings.
    ///
    /// ## Errors
    /// If the booking does not exist, or a failure occurred with the database.
    async fn sum_persons_by_datetime(
        &mut self,
        slot_id: Uuid,
        datetime: NaiveDateTime,
    ) -> Result<i32, ApiError>;

    /// Attempts to sum the number of persons by event.
    ///
    /// # Parameters
    /// The id of the event to retrieve the bookings.
    /// The minimum datetime of the bookings.
    /// The maximum datetime of the bookings.
    ///
    /// ## Success
    /// The number of persons, 0 if no bookings.
    ///
    /// ## Errors
    /// If the booking does not exist, or a failure occurred with the database.
    async fn sum_persons_by_event(
        &mut self,
        event_id: Uuid,
        min_date_time: NaiveDateTime,
        max_date_time: NaiveDateTime,
    ) -> Result<i32, ApiError>;
}
