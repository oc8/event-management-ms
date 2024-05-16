use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;
use protos::booking::v1::TimeData;
use crate::models::slot::Slot;

use crate::schema::{bookings};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = bookings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Booking {
    pub id: Uuid,
    pub slot_id: Uuid,
    pub booking_holder_key: String,
    pub date_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = bookings)]
pub struct NewBooking<'a> {
    pub slot_id: &'a Uuid,
    pub booking_holder_key: &'a str,
    pub date_time: &'a NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct BookingWithSlot {
    pub booking: Booking,
    pub slot: Slot,
}

impl BookingWithSlot {
    pub fn new(booking: Booking, slot: Slot) -> Self {
        BookingWithSlot { booking, slot }
    }
}

// TODO: limit database calls and clean some side effects in this code
impl Booking {
    pub fn create(
        conn: &mut PgConnection,
        booking: NewBooking,
    ) -> Result<BookingWithSlot, diesel::result::Error> {
        match diesel::insert_into(bookings::table)
            .values(booking)
            .returning(Booking::as_returning())
            .get_result(conn)
        {
            Ok(booking) => {
                let slot = Slot::find_by_id(conn, booking.slot_id).unwrap();
                Ok(BookingWithSlot::new(booking, slot))
            },
            Err(e) => {
                log::error!("Failed to create booking: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Option<BookingWithSlot> {
        let booking = bookings::dsl::bookings
            .select(Booking::as_select())
            .filter(bookings::dsl::id.eq(id))
            .first(conn)
            .ok();

        // TODO: Refactor this
        match booking {
            Some(b) => {
                let slot = Slot::find_by_id(conn, b.slot_id);
                match slot {
                    Some(s) => Some(BookingWithSlot::new(b, s)),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn find_by_slot_and_date_time(conn: &mut PgConnection, slot_id: Uuid, date_time: NaiveDateTime) -> Option<Booking> {
        bookings::dsl::bookings
            .select(Booking::as_select())
            .filter(bookings::dsl::slot_id.eq(slot_id))
            .filter(bookings::dsl::date_time.eq(date_time))
            .first(conn)
            .ok()
    }

    pub fn count_bookings_by_datetime(conn: &mut PgConnection, slot_id: Uuid, date_time: NaiveDateTime) -> i64 {
        bookings::dsl::bookings
            .select(diesel::dsl::count(bookings::dsl::id))
            .filter(bookings::dsl::slot_id.eq(slot_id))
            .filter(bookings::dsl::date_time.eq(date_time))
            .first(conn)
            .unwrap()
    }
}

impl From<Booking> for protos::booking::v1::Booking {
    fn from(booking: Booking) -> Self {
        let mut proto_booking = protos::booking::v1::Booking::default();

        proto_booking.id = booking.id.to_string();
        proto_booking.slot_id = booking.slot_id.to_string();
        proto_booking.booking_holder_key = booking.booking_holder_key;
        proto_booking.date_time = Some(TimeData {
            timezone: "UTC".to_string(),
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(booking.date_time, Utc).to_rfc3339()
        });
        proto_booking.created_at = booking.created_at.and_utc().timestamp();
        proto_booking.updated_at = booking.updated_at.and_utc().timestamp();

        proto_booking
    }
}

impl From<BookingWithSlot> for protos::booking::v1::Booking {
    fn from(booking_with_slots: BookingWithSlot) -> Self {
        let mut proto_booking = protos::booking::v1::Booking::from(booking_with_slots.booking);

        proto_booking.slot = Some(booking_with_slots.slot.into());
        proto_booking
    }
}