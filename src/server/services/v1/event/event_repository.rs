use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use rrule::RRuleSet;
use sqlx::{Acquire, PgConnection, Postgres, QueryBuilder};
use uuid::Uuid;
use crate::errors::{ApiError};
use crate::server::services::v1::event::event_model::{DbEvent, Event, EventActions, EventInsert, EventRepository, EventStatus, EventType, EventUpdate};
use crate::server::services::v1::slot::slot_model::{Slot, SlotRepository};
use crate::{format_datetime, naive_datetime_to_rrule_datetime, report_error, truncate_to_minute};
use crate::utils::filters::{EventFilters, Filters};
use protos::event::v1::{EventStatus as EventStatusProto, EventType as EventTypeProto};

#[async_trait]
impl EventRepository for PgConnection {
    async fn create_event(&mut self, event: &EventInsert) -> Result<Event, ApiError> {
        let conn = self.acquire().await?;

        let event = sqlx::query_as!(
            DbEvent,
            r#"
            INSERT INTO events (
                name, status, event_type, start_time, end_time, recurrence_rule, timezone,
                organizer_key, canceled_by, canceled_at, canceled_reason, slot_duration,
                capacity, slot_capacity
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, name, status AS "status: _", event_type AS "event_type: _", start_time, end_time, recurrence_rule, timezone,
                organizer_key, canceled_by, canceled_at, canceled_reason, slot_duration, capacity, slot_capacity, created_at, updated_at
            "#,
            event.name,
            event.status as EventStatus,
            event.event_type as EventType,
            truncate_to_minute(&event.start_time),
            truncate_to_minute(&event.end_time),
            event.recurrence_rule,
            event.timezone,
            event.organizer_key,
            event.canceled_by,
            event.canceled_at,
            event.canceled_reason,
            event.slot_duration,
            event.capacity,
            event.slot_capacity,
        )
            .fetch_one(&mut *conn)
            .await?;

        log::debug!("Created event: {:?}", event);

        let slots: Option<Vec<Slot>> = match event.event_type {
            EventType::Meeting => Some(self.generate_event_slots(&event).await?),
            _ => None
        };

        Ok(event.into_event(slots))
    }

    async fn get_event_by_id(&mut self, id: Uuid) -> Result<Event, ApiError> {
        let conn = self.acquire().await?;

        let event = sqlx::query_as!(
            DbEvent,
            r#"
            SELECT id, name, status AS "status: _", event_type AS "event_type: _", start_time, end_time, recurrence_rule, timezone,
                organizer_key, canceled_by, canceled_at, canceled_reason, slot_duration, capacity, slot_capacity, created_at, updated_at
            FROM events
            WHERE id = $1
            "#,
            id
        )
            .fetch_one(&mut *conn)
            .await?;

        log::debug!("Found event: {:?}", event);

        let slots: Option<Vec<Slot>> = match event.event_type {
            EventType::Meeting => Some(self.find_by_event_id(id).await?),
            _ => None
        };

        Ok(event.into_event(slots))
    }

    // async fn get_event_by_id(&mut self, id: Uuid) -> Result<Event, ApiError> {
    //     let mut conn = self.acquire().await?;
    //
    //     let rows = sqlx::query!(
    //         r#"
    //         SELECT
    //             e.id AS event_id,
    //             e.name,
    //             e.status AS "status: String",
    //             e.event_type AS "event_type: String",
    //             e.start_time,
    //             e.end_time,
    //             e.recurrence_rule,
    //             e.timezone,
    //             e.organizer_key,
    //             e.canceled_by,
    //             e.canceled_at,
    //             e.canceled_reason,
    //             e.slot_duration,
    //             e.capacity,
    //             e.slot_capacity,
    //             e.created_at,
    //             e.updated_at,
    //             s.id AS slot_id,
    //             s.start_time AS slot_start_time,
    //             s.end_time AS slot_end_time,
    //             s.capacity AS s_slot_capacity
    //         FROM
    //             events e
    //         LEFT JOIN
    //             event_slots s
    //         ON
    //             e.id = s.event_id
    //         WHERE
    //             e.id = $1
    //         "#,
    //         id
    //     )
    //         .fetch_all(&mut *conn)
    //         .await?;
    //
    //     let mut event: Option<DbEvent> = None;
    //     let mut slots: Vec<Slot> = Vec::new();
    //
    //     for row in rows {
    //         if event.is_none() {
    //             event = Some(DbEvent {
    //                 id: row.event_id,
    //                 name: row.name,
    //                 status: EventStatus::Active,
    //                 event_type: EventType::Meeting,
    //                 start_time: row.start_time,
    //                 end_time: row.end_time,
    //                 recurrence_rule: row.recurrence_rule,
    //                 timezone: row.timezone,
    //                 organizer_key: row.organizer_key,
    //                 canceled_by: row.canceled_by,
    //                 canceled_at: row.canceled_at,
    //                 canceled_reason: row.canceled_reason,
    //                 slot_duration: row.slot_duration,
    //                 capacity: row.capacity,
    //                 slot_capacity: row.slot_capacity,
    //                 created_at: row.created_at,
    //                 updated_at: row.updated_at,
    //             });
    //         }
    //
    //         if let (slot_id, slot_start_time, slot_end_time, slot_capacity) =
    //             (row.slot_id, row.slot_start_time, row.slot_end_time, row.s_slot_capacity)
    //         {
    //             slots.push(Slot {
    //                 id: slot_id,
    //                 event_id: row.event_id,
    //                 status: SlotStatus::Available,
    //                 event: None,
    //                 start_time: slot_start_time,
    //                 end_time: slot_end_time,
    //                 capacity: slot_capacity,
    //                 created_at: row.created_at,
    //                 updated_at: row.updated_at,
    //             });
    //         }
    //     }
    //
    //     let event = event.ok_or(EVENT_NOT_FOUND)?;
    //
    //     Ok(event.into_event(Some(slots)))
    // }

    async fn get_events_with_filter(
        &mut self,
        filters: &Filters<EventFilters>,
    ) -> Result<Vec<Event>, ApiError> {
        let conn = self.acquire().await?;

        log::debug!("Finding events with filters: {:?}", filters);

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT * FROM events
            WHERE 1 = 1
            "#,
        );

        if let Some(ref organizer_key) = filters.organizer_key {
            query_builder.push(" AND organizer_key = ");
            query_builder.push_bind(organizer_key);
        }
        if let Some(status) = filters.type_filters.status {
            if status != EventStatusProto::Unspecified {
                query_builder.push(" AND status = ");
                query_builder.push_bind(EventStatus::from_proto(status as i32));
            }
        }
        if let Some(event_type) = filters.type_filters.event_type {
            if event_type != EventTypeProto::Unspecified {
                query_builder.push(" AND event_type = ");
                query_builder.push_bind(EventType::from_proto(event_type as i32));
            }
        }
        if let Some(ref from) = filters.from {
            query_builder.push(" AND (start_time >= ");
            query_builder.push_bind(from);
            query_builder.push(" OR recurrence_rule IS NOT NULL)");
        }
        if let Some(ref to) = filters.to {
            query_builder.push(" AND start_time <= ");
            query_builder.push_bind(to);
        }

        query_builder.push(" ORDER BY start_time DESC");

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        let events = query_builder.build_query_as::<DbEvent>()
            .fetch_all(&mut *conn).await?;

        let mut result: Vec<Event> = Vec::new();
        for event in events {
            let slots: Option<Vec<Slot>> = match event.event_type {
                EventType::Meeting => Some(self.find_by_event_id(event.id).await?),
                _ => None
            };

            result.push(event.into_event(slots));
        }

        Ok(result)
    }

    async fn update_event(&mut self, id: Uuid, event: &EventUpdate) -> Result<Event, ApiError> {
        let conn = self.acquire().await?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            UPDATE events
            "#,
        );

        // TODO: This is not the best way to build the query but it works for now
        let mut is_first = true;

        if let Some(name) = &event.name {
            if !name.is_empty() {
                if is_first {
                    query_builder.push(" SET name = ");
                    is_first = false;
                } else {
                    query_builder.push(", name = ");
                }
                query_builder.push_bind(name);
            }
        }
        if let Some(status) = event.status {
            if status != EventStatus::Unspecified {
                if is_first {
                    query_builder.push(" SET status = ");
                    is_first = false;
                } else {
                    query_builder.push(", status = ");
                }
                query_builder.push_bind(status);
            }
        }
        if let Some(timezone) = &event.timezone {
            if !timezone.is_empty() {
                if is_first {
                    query_builder.push(" SET timezone = ");
                    is_first = false;
                } else {
                    query_builder.push(", timezone = ");
                }
                query_builder.push_bind(timezone);
            }
        }
        if let Some(start_time) = &event.start_time {
            if is_first {
                query_builder.push(" SET start_time = ");
                is_first = false;
            } else {
                query_builder.push(", start_time = ");
            }
            query_builder.push_bind(truncate_to_minute(start_time));
        }
        if let Some(end_time) = &event.end_time {
            if is_first {
                query_builder.push(" SET end_time = ");
                is_first = false;
            } else {
                query_builder.push(", end_time = ");
            }
            query_builder.push_bind(truncate_to_minute(end_time));
        }
        if let Some(canceled_at) = &event.canceled_at {
            if is_first {
                query_builder.push(" SET canceled_at = ");
                is_first = false;
            } else {
                query_builder.push(", canceled_at = ");
            }
            query_builder.push_bind(canceled_at);
        }
        if let Some(canceled_by) = &event.canceled_by {
            if is_first {
                query_builder.push(" SET canceled_by = ");
                is_first = false;
            } else {
                query_builder.push(", canceled_by = ");
            }
            query_builder.push_bind(canceled_by);
        }
        if let Some(canceled_reason) = &event.canceled_reason {
            if is_first {
                query_builder.push(" SET canceled_reason = ");
                is_first = false;
            } else {
                query_builder.push(", canceled_reason = ");
            }
            query_builder.push_bind(canceled_reason);
        }
        if let Some(capacity) = event.capacity {
            if capacity > 0 {
                if is_first {
                    query_builder.push(" SET capacity = ");
                    is_first = false;
                } else {
                    query_builder.push(", capacity = ");
                }
                query_builder.push_bind(capacity);
            }
        }
        if let Some(recurrence_rule) = &event.recurrence_rule {
            if !recurrence_rule.is_empty() {
                if is_first {
                    query_builder.push(" SET recurrence_rule = ");
                } else {
                    query_builder.push(", recurrence_rule = ");
                }
                query_builder.push_bind(recurrence_rule);
            }
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        query_builder.push(" RETURNING *");

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        let event = query_builder.build_query_as::<DbEvent>()
            .fetch_one(&mut *conn)
            .await?;

        let slots: Option<Vec<Slot>> = match event.event_type {
            EventType::Meeting => Some(self.find_by_event_id(id).await?),
            _ => None
        };

        Ok(event.into_event(slots))
    }

    async fn delete_event(&mut self, id: Uuid) -> Result<u64, ApiError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM events
            WHERE id = $1
            "#,
            id
        )
            .execute(self)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound("Event not found".to_string()));
        }

        log::debug!("Deleted event with id: {}", id);

        Ok(result.rows_affected())
    }
}

impl EventActions for Event {
    fn get_available_dates(&self, start: NaiveDateTime, limit: u16) -> Result<Vec<NaiveDate>, ApiError> {
        if self.recurrence_rule.is_none() {
            return Ok(vec![self.start_time.date()]);
        }

        let recurrence_rule = format!("DTSTART:{}\nRRULE:{}", format_datetime(self.start_time), self.recurrence_rule.clone().unwrap());
        let recurrence = recurrence_rule.parse::<RRuleSet>();

        log::debug!("Recurrence rule: {}", recurrence_rule);

        match recurrence {
            Ok(recurrence) => {
                let after = naive_datetime_to_rrule_datetime(start).unwrap();
                let rrule = recurrence.after(after);
                Ok(rrule.all(limit).dates
                    .into_iter()
                    .map(|date| date.naive_utc().date())
                    .collect())
            },
            Err(e) => {
                report_error(&e);
                Err(ApiError::InvalidRequest("Invalid recurrence rule".to_string()))
            }
        }
    }
}