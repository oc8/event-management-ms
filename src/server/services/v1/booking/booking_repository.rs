use async_trait::async_trait;
use sqlx::{PgConnection, Postgres, QueryBuilder};
use uuid::Uuid;
use crate::errors::{ApiError, INTERNAL};
use crate::server::services::v1::booking::booking_model::{Booking, BookingInsert, BookingRepository, DbBooking};
use crate::utils::filters::{BookingFilters, Filters};

#[async_trait]
impl BookingRepository for PgConnection {
    async fn create_booking(
        &mut self,
        booking: &BookingInsert,
    ) -> Result<Booking, ApiError> {
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
            .fetch_one(self)
            .await?;

        Ok(new_booking.into_booking(None))
    }

    async fn get_booking_by_id(&mut self, id: Uuid) -> Result<Booking, ApiError> {
        let booking = sqlx::query_as!(
            DbBooking,
            r#"
            SELECT * FROM bookings WHERE id = $1
            "#,
            id
        )
            .fetch_one(self)
            .await?;

        Ok(booking.into_booking(None))
    }

    async fn get_bookings_with_filters(&mut self, filters: &Filters<BookingFilters>) -> Result<Vec<Booking>, ApiError> {
        log::debug!("Finding bookings with filters: {:?}", filters);

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT * FROM bookings
            WHERE 1 = 1
            "#,
        );

        // if let Some(ref organizer_key) = filters.organizer_key {
        //     query_builder.push(" AND organizer_key = ");
        //     query_builder.push_bind(organizer_key);
        // }
        if let Some(ref from) = filters.from {
            query_builder.push(" AND date_time >= ");
            query_builder.push_bind(from);
        }
        if let Some(ref to) = filters.to {
            query_builder.push(" AND date_time <= ");
            query_builder.push_bind(to);
        }
        // if let Some(ref slot_id) = filters.type_filters.slot_id {
        //     query_builder.push(" AND slot_id = ");
        //     query_builder.push_bind(slot_id);
        // }
        if let Some(ref booking_holder_key) = filters.type_filters.booking_holder_key {
            query_builder.push(" AND booking_holder_key = ");
            query_builder.push_bind(booking_holder_key);
        }

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        let bookings = query_builder.build_query_as::<DbBooking>();

        let bookings = bookings.fetch_all(self).await
            .map_err(|e| {
                log::error!("Failed to fetch bookings: {:?}", e);
                INTERNAL
            })?;

        Ok(bookings.into_iter().map(|b| b.into_booking(None)).collect())
    }

    async fn delete_booking(&mut self, _id: Uuid) -> Result<usize, ApiError> {
        todo!()
    }
}