use crate::database::PgPooledConnection;
use crate::errors::ApiError;
use crate::server::services::v1::booking::booking_model::BookingRepository;
use crate::server::services::v1::closure::closure_model::ClosureRepository;
use crate::server::services::v1::event::event_model::{
    EventInsert, EventRepository, EventStatus, EventType, EventUpdate,
};
use crate::server::services::v1::event::timeline::Timeline;
use crate::utils::filters::{BookingFilters, ClosureFilters, EventFilters, Filters};
use crate::utils::validation::ValidateRequest;
use crate::{get_meta_timezone, truncate_to_minute};
use chrono::DateTime;
use event_protos::event::v1::{
    CancelEventRequest, CancelEventResponse, CreateEventRequest, CreateEventResponse,
    DeleteEventRequest, DeleteEventResponse, GetEventRequest, GetEventResponse, GetTimelineRequest,
    GetTimelineResponse, ListEventsRequest, ListEventsResponse, UpdateEventRequest,
    UpdateEventResponse,
};
use sqlx::postgres::types::PgInterval;
use tonic::metadata::MetadataMap;
use uuid::Uuid;

pub async fn create_event(
    request: CreateEventRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<CreateEventResponse, ApiError> {
    request.validate()?;

    let start_time = DateTime::parse_from_rfc3339(&request.start)?;
    let end_time = DateTime::parse_from_rfc3339(&request.end)?;

    let interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: request.slot_duration * 60_000_000,
    };

    // TODO: make all modifiers inside the create_event method
    let event = conn
        .create_event(&EventInsert {
            name: request.name.to_string(),
            status: EventStatus::Active,
            event_type: EventType::from_proto(request.event_type),
            start_time: truncate_to_minute(&start_time.naive_utc()),
            end_time: truncate_to_minute(&end_time.naive_utc()),
            recurrence_rule: match request.recurrence_rule.is_empty() {
                true => None,
                false => Some(request.recurrence_rule),
            },
            organizer_key: request.organizer_key.to_string(),
            canceled_by: None,
            canceled_at: None,
            canceled_reason: None,
            slot_duration: Some(interval),
            capacity: Some(request.capacity),
            slot_capacity: match request.slot_capacity {
                0 => Some(1),
                _ => Some(request.slot_capacity),
            },
        })
        .await?;

    log::debug!("event created: {:?}", event);

    Ok(CreateEventResponse {
        event: Some(event.to_response(get_meta_timezone(meta))),
    })
}

pub async fn get_event_by_id(
    request: GetEventRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<GetEventResponse, ApiError> {
    request.validate()?;

    let event = conn
        .get_event_by_id(Uuid::parse_str(&request.id).unwrap())
        .await?;

    Ok(GetEventResponse {
        event: Some(event.to_response(get_meta_timezone(meta))),
    })
}

pub async fn list_events(
    request: ListEventsRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<ListEventsResponse, ApiError> {
    request.validate()?;

    let filters: Filters<EventFilters> = request.filters.into();

    let events = conn.get_events_with_filter(&filters).await?;

    Ok(ListEventsResponse {
        events: events
            .into_iter()
            .map(|e| e.to_response(get_meta_timezone(meta)))
            .collect(),
    })
}

pub async fn update_event(
    request: UpdateEventRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<UpdateEventResponse, ApiError> {
    request.validate()?;

    let event_id = Uuid::parse_str(&request.id)?;

    let start_time = match request.start.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.start)?.naive_utc()),
    };

    let end_time = match request.end.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.end)?.naive_utc()),
    };

    let event = conn
        .update_event(
            event_id,
            &EventUpdate {
                name: Some(request.name),
                status: None,
                start_time,
                end_time,
                recurrence_rule: Some(request.recurrence_rule),
                canceled_by: None,
                canceled_at: None,
                canceled_reason: None,
                capacity: Some(request.capacity),
                slot_capacity: Some(request.slot_capacity),
            },
        )
        .await?;

    Ok(UpdateEventResponse {
        event: Some(event.to_response(get_meta_timezone(meta))),
    })
}

pub async fn delete_event(
    request: DeleteEventRequest,
    conn: &mut PgPooledConnection,
) -> Result<DeleteEventResponse, ApiError> {
    request.validate()?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    conn.delete_event(event_id).await?;

    Ok(DeleteEventResponse {
        message: "Event deleted successfully".to_string(),
    })
}

pub async fn cancel_event(
    request: CancelEventRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<CancelEventResponse, ApiError> {
    request.validate()?;

    let event_id = Uuid::parse_str(&request.id).unwrap();

    let event = conn
        .update_event(
            event_id,
            &EventUpdate {
                status: Some(EventStatus::Canceled),
                canceled_by: Some(request.canceled_by),
                canceled_at: Some(chrono::Utc::now().naive_utc()),
                canceled_reason: Some(request.reason),
                ..Default::default()
            },
        )
        .await?;

    Ok(CancelEventResponse {
        event: Some(event.to_response(get_meta_timezone(meta))),
    })
}

pub async fn get_timeline(
    request: GetTimelineRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<GetTimelineResponse, ApiError> {
    request.validate()?;

    let event_filters: Filters<EventFilters> = request.filters.clone().into();
    let closure_filters: Filters<ClosureFilters> = request.filters.clone().into();
    let booking_filters: Filters<BookingFilters> = request.filters.clone().into();

    let mut events = conn.get_events_with_filter(&event_filters).await?;
    let closures = conn.get_closures_with_filters(&closure_filters).await?;
    let bookings = conn.get_bookings_with_filters(&booking_filters).await?;

    let mut timeline = Timeline::new(events.clone(), closures.clone(), bookings);

    events = timeline.included(event_filters.from.unwrap(), event_filters.to.unwrap())?;

    Ok(GetTimelineResponse {
        events: events
            .into_iter()
            .map(|e| e.to_response(get_meta_timezone(meta)))
            .collect(),
    })
}
