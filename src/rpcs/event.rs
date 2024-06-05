use diesel::data_types::PgInterval;
use tonic::Status;
use uuid::Uuid;
use booking_ms::report_error;
use protos::booking::v1::{CancelEventRequest, CancelEventResponse, CreateEventRequest, CreateEventResponse, DeleteEventRequest, DeleteEventResponse, EventStatus, EventType, GetEventRequest, GetEventResponse, UpdateEventRequest, UpdateEventResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::booking::{Booking, BookingWithSlot};
use crate::models::closure::Closure;
use crate::models::event::{Event, NewEvent};
use crate::models::filters::{BookingFilters, EventFilters, Filters};
use crate::models::timeline::Timeline;
use crate::validations::{validate_cancel_event_request, validate_create_event_request, validate_delete_event_request, validate_get_event_request, validate_update_event_request};

pub fn create_event(
    request: CreateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateEventResponse, Status> {
    validate_create_event_request(&request)?;

    let start_time = chrono::NaiveDateTime::parse_from_str(&request.start, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_DATETIME))?;
    let end_time = chrono::NaiveDateTime::parse_from_str(&request.end, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_DATETIME))?;

    let tz = match request.timezone.is_empty() {
        true => chrono::Utc.to_string(),
        false => request.timezone.to_string()
    };

    let interval = PgInterval::new(request.slot_duration * 60_000_000, 0, 0);

    let event_type = match EventType::try_from(request.event_type) {
        Ok(event_type) => event_type.as_str_name(),
        Err(_) => EventType::Event.as_str_name()
    };

    let new_event = NewEvent {
        name: &request.name.to_string(),
        status: EventStatus::Active.as_str_name(),
        event_type,
        timezone: &tz,
        start_time: &start_time,
        end_time: &end_time,
        organizer_key: &request.organizer_key.to_string(),
        canceled_at: None,
        canceled_by: None,
        canceled_reason: None,
        slot_duration: Some(&interval),
        recurrence_rule: match request.recurrence_rule.is_empty() {
            true => None,
            false => Some(&request.recurrence_rule)
        },
        slot_capacity: match request.slot_capacity {
            0 => Some(1),
            _ => Some(request.slot_capacity)
        },
        capacity: Some(request.capacity),
    };

    let event = Event::create(conn, new_event)
        .map_err(|e| {
            report_error(&e);
            format_error(errors::EVENT_CREATION_FAILED)
        })?;

    log::debug!("event created: {:?}", event);

    Ok(CreateEventResponse{
        event: Some(event.into())
    })
}

pub fn get_event_by_id(
    request: GetEventRequest,
    conn: &mut PgPooledConnection
) -> Result<GetEventResponse, Status> {
    validate_get_event_request(&request)?;

    let event = Event::find_by_id(conn, Uuid::parse_str(&request.id).unwrap())
        .ok_or_else(|| format_error(errors::EVENT_NOT_FOUND))?;

    Ok(GetEventResponse{
        event: Some(event.into())
    })
}

pub fn update_event(
    request: UpdateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<UpdateEventResponse, Status> {
    validate_update_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    let event = Event::find_by_id(conn, event_id)
        .ok_or_else(|| format_error(errors::EVENT_NOT_FOUND))?;

    let name = match request.name.is_empty() {
        true => event.event.name,
        false => request.name.to_string()
    };

    let start_time = match request.start.is_empty() {
        true => event.event.start_time,
        false => chrono::NaiveDateTime::parse_from_str(&request.start, "%Y-%m-%dT%H:%M:%S")
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
    };

    let end_time = match request.end.is_empty() {
        true => event.event.end_time,
        false => chrono::NaiveDateTime::parse_from_str(&request.end, "%Y-%m-%dT%H:%M:%S")
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
    };

    let tz = match request.timezone.is_empty() {
        true => event.event.timezone,
        false => request.timezone.to_string()
    };

    let recurrence_rule = match request.recurrence_rule.is_empty() {
        true => event.event.recurrence_rule,
        false => Some(request.recurrence_rule)
    };

    let slot_capacity = match request.slot_capacity == 0 {
        true => event.event.slot_capacity,
        false => Some(request.slot_capacity)
    };

    let capacity = match request.capacity == 0 {
        true => event.event.capacity,
        false => Some(request.capacity)
    };

    let updated_event = Event::update(conn, event_id, NewEvent{
        name: &name,
        status: &event.event.status,
        event_type: &event.event.event_type,
        timezone: &tz,
        start_time: &start_time,
        end_time: &end_time,
        organizer_key: &event.event.organizer_key,
        canceled_at: match event.event.canceled_at.is_some() {
            true => Some(event.event.canceled_at.as_ref().unwrap()),
            false => None
        },
        canceled_by: match event.event.canceled_by.is_some() {
            true => Some(event.event.canceled_by.as_ref().unwrap()),
            false => None
        },
        canceled_reason: match event.event.canceled_reason.is_some() {
            true => Some(event.event.canceled_reason.as_ref().unwrap()),
            false => None
        },
        slot_duration: match event.event.slot_duration.is_some() {
            true => Some(event.event.slot_duration.as_ref().unwrap()),
            false => None
        },
        recurrence_rule: match recurrence_rule.is_some() {
            true => Some(recurrence_rule.as_ref().unwrap()),
            false => None
        },
        slot_capacity,
        capacity,
    }).map_err(|e| {
        report_error(&e);
        format_error(errors::EVENT_UPDATE_FAILED)
    })?;

    Ok(UpdateEventResponse{
        event: Some(updated_event.event.into())
    })
}

pub fn delete_event(
    request: DeleteEventRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteEventResponse, Status> {
    validate_delete_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    Event::find_by_id(conn, event_id)
        .ok_or_else(|| format_error(errors::EVENT_NOT_FOUND))?;

    let _ = Event::delete(conn, event_id).map_err(|e| {
        report_error(&e);
        format_error(errors::EVENT_DELETION_FAILED)
    })?;

    Ok(DeleteEventResponse{
        message: "Event deleted successfully".to_string()
    })
}

pub fn cancel_event(
    request: CancelEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CancelEventResponse, Status> {
    validate_cancel_event_request(&request)?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    let event = Event::find_by_id(conn, event_id)
        .ok_or_else(|| format_error(errors::EVENT_NOT_FOUND))?;

    let canceled_at = chrono::Utc::now().naive_utc();

    let updated_event = Event::update(conn, event_id, NewEvent {
        name: &event.event.name,
        status: EventStatus::Canceled.as_str_name(),
        event_type: &event.event.event_type,
        timezone: &event.event.timezone,
        start_time: &event.event.start_time,
        end_time: &event.event.end_time,
        organizer_key: &event.event.organizer_key,
        canceled_at: Some(&canceled_at),
        canceled_by: match request.canceled_by.is_empty() {
            true => None,
            false => Some(&request.canceled_by)
        },
        canceled_reason: match request.reason.is_empty() {
            true => None,
            false => Some(&request.reason)
        },
        slot_duration: Option::from(&event.event.slot_duration),
        recurrence_rule: match event.event.recurrence_rule.is_some() {
            true => Some(event.event.recurrence_rule.as_ref().unwrap()),
            false => None
        },
        slot_capacity: match event.event.slot_capacity.is_some() {
            true => Some(event.event.slot_capacity.unwrap()),
            false => None
        },
        capacity: match event.event.capacity.is_some() {
            true => Some(event.event.capacity.unwrap()),
            false => None
        }
    }).map_err(|e| {
        report_error(&e);
        format_error(errors::EVENT_UPDATE_FAILED)
    })?;

    Ok(CancelEventResponse{
        event: Some(updated_event.event.into())
    })
}