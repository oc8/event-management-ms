use diesel::data_types::PgInterval;
use tonic::Status;
use protos::booking::v1::{CreateEventRequest, CreateEventResponse, EventStatus, EventType};
use crate::database::PgPooledConnection;
use crate::errors::errors;
use crate::models::event::{Event, NewEvent};
use crate::validations::validate_create_event_request;

pub fn create_event(
    request: CreateEventRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateEventResponse, Status> {
    validate_create_event_request(&request)?;

    let start_time = chrono::NaiveDateTime::parse_from_str(&request.start, "%Y-%m-%dT%H:%M")
        .map_err(|_| Status::invalid_argument(errors::INVALID_EVENT_START_DATE))?;
    let end_time = chrono::NaiveDateTime::parse_from_str(&request.end, "%Y-%m-%dT%H:%M")
        .map_err(|_| Status::invalid_argument(errors::INVALID_EVENT_END_DATE))?;

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
        max_guests: Some(&request.max_guests),
        recurrence_rule: match request.recurrence_rule.is_empty() {
            true => None,
            false => Some(&request.recurrence_rule)
        },
        max_persons_per_slot: Some(&request.max_guests),
    };

    let event = Event::create(conn, new_event)
        .map_err(|_| Status::internal("Failed to create event"))?;

    log::info!("Event created: {:?}", event);

    Ok(CreateEventResponse{
        event: Some(event.into())
    })
}
