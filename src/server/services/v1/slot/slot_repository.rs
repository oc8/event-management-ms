use async_trait::async_trait;
use uuid::Uuid;
use protos::event::v1::SlotStatus;
use crate::errors::ApiError;
use crate::server::services::v1::event::event_model::{DbEvent};
use crate::server::services::v1::slot::slot_model::{DbSlot, Slot, SlotRepository};
use sqlx::{PgConnection};

#[async_trait]
impl SlotRepository for PgConnection {
    async fn get_slot_by_id(&mut self, id: Uuid) -> Result<Slot, ApiError> {
        let slot = sqlx::query_as!(
            DbSlot,
            r#"
            SELECT * FROM event_slots WHERE id = $1
            "#,
            id
        )
        .fetch_one(self)
        .await?;

        Ok(slot.into_slot(SlotStatus::Available, None))
    }

    async fn find_by_event_id(&mut self, event_id: Uuid) -> Result<Vec<Slot>, ApiError> {
        let rows = sqlx::query_as!(
            DbSlot,
            r#"
            SELECT * FROM event_slots WHERE event_id = $1
            "#,
            event_id
        )
        .fetch_all(self)
        .await?;

        Ok(rows.into_iter().map(|row| row.into_slot(SlotStatus::Available, None)).collect())
    }

    async fn generate_event_slots(&mut self, event: &DbEvent) -> Result<Vec<Slot>, ApiError> {
        let slots = sqlx::query_as!(
            DbSlot,
            r#"
            WITH RECURSIVE slot_times AS (
                SELECT
                $1::TIMESTAMP AS slot_start_time,
                $1::TIMESTAMP + INTERVAL '1 minute' * $3 AS slot_end_time
                UNION ALL
                SELECT
                slot_end_time,
                slot_end_time + INTERVAL '1 minute' * $3
                FROM
                slot_times
                WHERE
                slot_end_time < $2
            )
            INSERT INTO event_slots (event_id, start_time, end_time, capacity)
            SELECT
            $4 AS event_id,
            slot_start_time,
            slot_end_time,
            $5 AS capacity
            FROM
            slot_times
            RETURNING *;
            "#,
            event.start_time,
            event.end_time,
            (event.slot_duration.clone().unwrap().microseconds / 60_000_000) as i32,
            event.id,
            event.capacity
        )
            .fetch_all(self)
            .await?;

        Ok(slots.into_iter().map(|slot| slot.into_slot(SlotStatus::Available, None)).collect())
    }
}
