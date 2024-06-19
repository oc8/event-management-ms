use chrono::DateTime;
use sqlx::postgres::types::PgInterval;
use tonic::Status;
use uuid::Uuid;
use protos::event::v1::{CancelEventRequest, CancelEventResponse, CreateEventRequest, CreateEventResponse, DeleteEventRequest, DeleteEventResponse, GetEventRequest, GetEventResponse, GetListEventsRequest, GetListEventsResponse, UpdateEventRequest, UpdateEventResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::server::services::v1::event::event_model::{EventInsert, EventRepository, EventStatus, EventType, EventUpdate};
use crate::{truncate_to_minute};
use crate::utils::filters::{EventFilters, Filters};

pub async fn create_event(
    request: CreateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateEventResponse, Status> {
    // validate_create_event_request(&request)?;

    let start_time = DateTime::parse_from_rfc3339(&request.start)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?;
    let end_time = DateTime::parse_from_rfc3339(&request.end)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?;

    let tz = match request.timezone.is_empty() {
        true => chrono::Utc.to_string(),
        false => request.timezone.to_string()
    };

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: request.slot_duration * 60_000_000,
    };

    // TODO: make all modifiers inside the create_event method
    let event = conn.create_event(&EventInsert{
        name: request.name.to_string(),
        status: EventStatus::Active,
        event_type: EventType::from_proto(request.event_type),
        start_time: truncate_to_minute(&start_time.naive_utc()),
        end_time: truncate_to_minute(&end_time.naive_utc()),
        recurrence_rule: match request.recurrence_rule.is_empty() {
            true => None,
            false => Some(request.recurrence_rule)
        },
        timezone: tz.to_string(),
        organizer_key: request.organizer_key.to_string(),
        canceled_by: None,
        canceled_at: None,
        canceled_reason: None,
        slot_duration: Some(interval),
        capacity: Some(request.capacity),
        slot_capacity: match request.slot_capacity {
            0 => Some(1),
            _ => Some(request.slot_capacity)
        },
    }).await?;

    log::debug!("event created: {:?}", event);

    Ok(CreateEventResponse{
        event: Some(event.into())
    })
}

pub async fn get_event_by_id(
    request: GetEventRequest,
    conn: &mut PgPooledConnection
) -> Result<GetEventResponse, Status> {
    // validate_get_event_request(&request)?;

    let event = conn.get_event_by_id(Uuid::parse_str(&request.id).unwrap())
        .await?;

    Ok(GetEventResponse{
        event: Some(event.into())
    })
}

pub async fn list_events(
    request: GetListEventsRequest,
    conn: &mut PgPooledConnection
) -> Result<GetListEventsResponse, Status> {
    // validate_get_event_request(&request)?;

    let filters: Filters<EventFilters> = request.filters.into();

    let events = conn.get_events_with_filter(&filters)
        .await?;

    println!("{:?}", events);

    Ok(GetListEventsResponse{
        events: events.into_iter().map(|e| e.into()).collect()
    })
}

pub async fn update_event(
    request: UpdateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<UpdateEventResponse, Status> {
    // validate_update_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    let start_time = match request.start.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.start)
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
            .naive_utc())
    };

    let end_time = match request.end.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.end)
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
            .naive_utc())
    };

    let event = conn.update_event(event_id, &EventUpdate{
        name: Some(request.name),
        status: None,
        start_time,
        end_time,
        recurrence_rule: Some(request.recurrence_rule),
        timezone: Some(request.timezone),
        canceled_by: None,
        canceled_at: None,
        canceled_reason: None,
        capacity: Some(request.capacity),
        slot_capacity: Some(request.slot_capacity),
    }).await?;

    Ok(UpdateEventResponse{
        event: Some(event.into())
    })
}

pub async fn delete_event(
    request: DeleteEventRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteEventResponse, Status> {
    // validate_delete_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    conn.delete_event(event_id)
        .await?;

    Ok(DeleteEventResponse{
        message: "Event deleted successfully".to_string()
    })
}

pub async fn cancel_event(
    request: CancelEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CancelEventResponse, Status> {
    // validate_cancel_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    let event = conn.update_event(event_id, &EventUpdate{
        status: Some(EventStatus::Canceled),
        canceled_by: Some(request.canceled_by),
        canceled_at: Some(chrono::Utc::now().naive_utc()),
        canceled_reason: Some(request.reason),
        ..Default::default()
    }).await?;

    Ok(CancelEventResponse{
        event: Some(event.into())
    })
}
