use diesel::data_types::PgInterval;
use log::error;
use tonic::Status;
use uuid::Uuid;
use booking_ms::report_error;
use protos::booking::v1::{CreateEventRequest, CreateEventResponse, EventStatus, EventType, GetActiveEventsInstancesRequest, GetActiveEventsInstancesResponse, GetEventInstancesRequest, GetEventInstancesResponse, GetEventRequest, GetEventResponse, ListEventsRequest, ListEventsResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::closure::Closure;
use crate::models::event::{Event, NewEvent};
use crate::models::event_instances::{EventInstances};
use crate::models::filters::Filters;
use crate::validations::{validate_create_event_request, validate_list_events, validate_get_active_events_instances, validate_get_event_instances, validate_get_event_request};

pub fn create_event(
    request: CreateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateEventResponse, Status> {
    validate_create_event_request(&request)?;

    let start_time = chrono::NaiveDateTime::parse_from_str(&request.start, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_EVENT_START_DATE))?;
    let end_time = chrono::NaiveDateTime::parse_from_str(&request.end, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_EVENT_END_DATE))?;

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
            0 => Some(&1),
            _ => Some(&request.slot_capacity)
        },
        capacity: Some(&request.capacity),
    };

    let event = Event::create(conn, new_event)
        .map_err(|e| {
            report_error(e);
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

pub fn list_events(
    request: ListEventsRequest,
    conn: &mut PgPooledConnection
) -> Result<ListEventsResponse, Status> {
    validate_list_events(&request)?;

    let filters: Filters = request.filters.into();

    let events = Event::find_events(conn, filters);

    Ok(ListEventsResponse{
        events: events.into_iter().map(|e| e.into()).collect()
    })
}

pub fn get_event_instances(
    request: GetEventInstancesRequest,
    conn: &mut PgPooledConnection
) -> Result<GetEventInstancesResponse, Status> {
    validate_get_event_instances(&request)?;

    let event_id = Uuid::parse_str(&request.event_id)
        .map_err(|_| format_error(errors::INVALID_EVENT_ID))?;

    let event = Event::find_by_id(conn, event_id)
        .ok_or_else(|| format_error(errors::EVENT_NOT_FOUND))?;

    let closures = Closure::find_by_organizer_key(conn, &event.event.organizer_key);

    let instances = EventInstances::new(event.event, event.slots, closures);

    Ok(GetEventInstancesResponse{
        event: Some(instances.into()),
    })
}

pub fn get_active_events_instances(
    request: GetActiveEventsInstancesRequest,
    conn: &mut PgPooledConnection
) -> Result<GetActiveEventsInstancesResponse, Status> {
    validate_get_active_events_instances(&request)?;

    let filters: Filters = request.filters.into();

    let events = Event::find_events(conn, filters);

    if events.is_empty() {
        return Ok(GetActiveEventsInstancesResponse{
            events: vec![]
        })
    }

    let organizer_key = events[0].event.organizer_key.clone();
    let closures = Closure::find_by_organizer_key(conn, &organizer_key);
    let events_instances: Vec<EventInstances> = events.iter()
        .map(|event| {
            EventInstances::new(event.event.clone(), event.slots.clone(), closures.clone())
        })
        .collect();

    Ok(GetActiveEventsInstancesResponse{
        events: events_instances.into_iter().map(|e| e.into()).collect()
    })
}