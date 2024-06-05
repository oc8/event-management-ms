use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper, QueryableByName};
use diesel::dsl::{sum};
use uuid::Uuid;
use protos::booking::v1::{TimeData};
use crate::models::filters::{BookingFilters, Filters};
use crate::models::slot::{DbSlot, Slot};

use crate::schema::{bookings, event_slots};

#[derive(Queryable, Selectable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = bookings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Booking {
    pub id: Uuid,
    pub slot_id: Uuid,
    pub booking_holder_key: String,
    pub organizer_key: String,
    pub date_time: NaiveDateTime,
    pub persons: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = bookings)]
pub struct NewBooking<'a> {
    pub slot_id: &'a Uuid,
    pub booking_holder_key: &'a str,
    pub organizer_key: &'a str,
    pub date_time: &'a NaiveDateTime,
    pub persons: &'a i32,
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
                let slot = DbSlot::find_by_id(conn, booking.slot_id).unwrap();
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

        booking.and_then(|b| {
            DbSlot::find_by_id(conn, b.slot_id).map(|s| BookingWithSlot::new(b, s))
        })
    }

    pub fn find(conn: &mut PgConnection, filters: &Filters<BookingFilters>) -> Vec<BookingWithSlot> {
        log::debug!("finding bookings with filters={:?}", filters);

        let mut query = bookings::dsl::bookings
            .select(Booking::as_select())
            .into_boxed();

        if let Some(from) = &filters.from {
            query = query.filter(bookings::dsl::date_time.ge(from));
        }
        if let Some(to) = &filters.to {
            query = query.filter(bookings::dsl::date_time.le(to));
        }

        // if let Some(slot_id) = &filters.type_filters.slot_id {
        //     query = query.filter(bookings::dsl::slot_id.eq(slot_id));
        // }

        if let Some(booking_holder_key) = &filters.type_filters.booking_holder_key {
            if !booking_holder_key.is_empty() {
                query = query.filter(bookings::dsl::booking_holder_key.eq(booking_holder_key));
            }
        }

        if let Some(organizer_key) = &filters.organizer_key {
            query = query.filter(bookings::dsl::organizer_key.eq(organizer_key));
        }

        log::debug!("query={:?}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

        query
            .load::<Booking>(conn)
            .expect("Error loading bookings")
            .into_iter()
            .map(|b| {
                let slot = DbSlot::find_by_id(conn, b.slot_id).unwrap();
                BookingWithSlot::new(b, slot)
            })
            .collect()
    }

    pub fn delete(conn: &mut PgConnection, id: Uuid) -> Result<usize, diesel::result::Error> {
        diesel::delete(bookings::dsl::bookings.filter(bookings::dsl::id.eq(id)))
            .execute(conn)
    }

    pub fn find_duplicated_booking(conn: &mut PgConnection, slot_id: Uuid, booking_holder: String, date_time: NaiveDateTime) -> Option<Booking> {
        bookings::dsl::bookings
            .select(Booking::as_select())
            .filter(bookings::dsl::slot_id.eq(slot_id))
            .filter(bookings::dsl::booking_holder_key.eq(booking_holder))
            .filter(bookings::dsl::date_time.eq(date_time))
            .first(conn)
            .ok()
    }

    pub fn sum_persons_by_datetime(conn: &mut PgConnection, slot_id: Uuid, date_time: NaiveDateTime) -> Option<i32> {
        let sum = bookings::dsl::bookings
            .filter(bookings::dsl::slot_id.eq(slot_id))
            .filter(bookings::dsl::date_time.eq(date_time))
            .select(diesel::dsl::sum(bookings::dsl::persons))
            .get_result::<Option<i64>>(conn).ok()?;

        sum.map(|s| s as i32)
    }

    pub fn sum_persons_by_event(conn: &mut PgConnection, event_id: Uuid, min_date_time: NaiveDateTime, max_date_time: NaiveDateTime) -> Option<i32> {
        let sum = bookings::dsl::bookings
            .inner_join(event_slots::table)
            .filter(event_slots::dsl::event_id.eq(event_id))
            .filter(bookings::dsl::date_time.ge(min_date_time))
            .filter(bookings::dsl::date_time.le(max_date_time))
            .select(sum(bookings::dsl::persons))
            .get_result::<Option<i64>>(conn)
            .ok()?;

        sum.map(|s| s as i32)
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
        proto_booking.persons = booking.persons;
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