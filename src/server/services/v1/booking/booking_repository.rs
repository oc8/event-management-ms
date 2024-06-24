use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{Acquire, PgConnection, Postgres, QueryBuilder};
use uuid::Uuid;
use crate::errors::{ApiError};
use crate::server::services::v1::booking::booking_model::{Booking, BookingInsert, BookingRepository, DbBooking};
use crate::server::services::v1::slot::slot_model::SlotRepository;
use crate::utils::filters::{BookingFilters, Filters};

#[async_trait]
impl BookingRepository for PgConnection {
    async fn create_booking(
        &mut self,
        booking: &BookingInsert,
    ) -> Result<Booking, ApiError> {
        let conn = self.acquire().await?;

        let new_booking = sqlx::query_as!(
            DbBooking,
            r#"
            INSERT INTO bookings (slot_id, date_time, organizer_key, persons, booking_holder_key)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            booking.slot_id,
            booking.date_time,
            booking.organizer_key,
            booking.persons,
            booking.booking_holder_key
        )
            .fetch_one(&mut *conn)
            .await?;

        let slot = self.get_slot_by_id(booking.slot_id).await?;

        log::debug!("Created booking: {:?}", new_booking);

        Ok(new_booking.into_booking(Some(slot)))
    }

    async fn get_booking_by_id(&mut self, id: Uuid) -> Result<Booking, ApiError> {
        let conn = self.acquire().await?;

        let booking = sqlx::query_as!(
            DbBooking,
            r#"
            SELECT * FROM bookings WHERE id = $1
            "#,
            id
        )
            .fetch_one(&mut *conn)
            .await?;

        let slot = conn.get_slot_by_id(booking.slot_id).await?;

        log::debug!("Found booking: {:?}", booking);

        Ok(booking.into_booking(Some(slot)))
    }

    async fn get_bookings_with_filters(&mut self, filters: &Filters<BookingFilters>) -> Result<Vec<Booking>, ApiError> {
        log::debug!("Finding bookings with filters: {:?}", filters);

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT * FROM bookings
            WHERE 1 = 1
            "#,
        );

        if let Some(ref organizer_key) = filters.organizer_key {
            query_builder.push(" AND organizer_key = ");
            query_builder.push_bind(organizer_key);
        }
        if let Some(ref from) = filters.from {
            query_builder.push(" AND date_time >= ");
            query_builder.push_bind(from);
        }
        if let Some(ref to) = filters.to {
            query_builder.push(" AND date_time <= ");
            query_builder.push_bind(to);
        }
        if let Some(ref slot_id) = filters.type_filters.slot_id {
            query_builder.push(" AND slot_id = ");
            query_builder.push_bind(slot_id);
        }
        if let Some(ref booking_holder_key) = filters.type_filters.booking_holder_key {
            if !booking_holder_key.is_empty() {
                query_builder.push(" AND booking_holder_key = ");
                query_builder.push_bind(booking_holder_key);
            }
        }

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        let bookings = query_builder.build_query_as::<DbBooking>();

        let bookings = bookings.fetch_all(self).await?;

        Ok(bookings.into_iter().map(|b| b.into_booking(None)).collect())
    }

    async fn delete_booking(&mut self, _id: Uuid) -> Result<usize, ApiError> {
        todo!()
    }

    async fn get_booking_holder_booking(&mut self, slot_id: Uuid, booking_holder: String, date_time: NaiveDateTime) -> Result<Booking, ApiError> {
        let booking = sqlx::query_as!(
            DbBooking,
            r#"
            SELECT * FROM bookings
            WHERE slot_id = $1 AND booking_holder_key = $2 AND date_time = $3
            "#,
            slot_id,
            booking_holder,
            date_time
        )
            .fetch_one(self)
            .await?;

        log::debug!("Found booking: {:?}", booking);

        Ok(booking.into_booking(None))
    }

    async fn sum_persons_by_datetime(&mut self, slot_id: Uuid, datetime: NaiveDateTime) -> Result<i32, ApiError> {
        let result: Option<i64> = sqlx::query_scalar!(
            "SELECT SUM(persons) FROM bookings WHERE slot_id = $1 AND date_time = $2",
            slot_id,
            datetime
        )
            .fetch_one(self)
            .await?;

        log::debug!("Summed persons: {:?}", result);

        if let Some(result) = result {
            Ok(result as i32)
        } else {
            Ok(0)
        }
    }

    async fn sum_persons_by_event(&mut self, event_id: Uuid, min_date_time: NaiveDateTime, max_date_time: NaiveDateTime) -> Result<i32, ApiError> {
        let result: Option<i64> = sqlx::query_scalar!(
            "SELECT SUM(bookings.persons)
             FROM bookings
             INNER JOIN event_slots ON bookings.slot_id = event_slots.id
             WHERE event_slots.event_id = $1
               AND bookings.date_time >= $2
               AND bookings.date_time <= $3",
            event_id,
            min_date_time,
            max_date_time
        )
            .fetch_one(self)
            .await?;

        log::debug!("Summed persons: {:?}", result);

        if let Some(result) = result {
            Ok(result as i32)
        } else {
            Ok(0)
        }
    }
}