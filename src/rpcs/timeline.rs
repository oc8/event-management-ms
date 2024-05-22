use tonic::Status;
use protos::booking::v1::{GetTimelineRequest, GetTimelineResponse};
use crate::database::PgPooledConnection;
use crate::models::closure::Closure;
use crate::models::event::Event;
use crate::models::filters::Filters;
use crate::models::timeline::Timeline;
use crate::validations::validate_get_timeline;

// THIS IS A TEST ROUTE, NEEDS TO BE REMOVED
pub fn get_timeline(
    request: GetTimelineRequest,
    conn: &mut PgPooledConnection
) -> Result<GetTimelineResponse, Status> {
    validate_get_timeline(&request)?;

    let from = chrono::NaiveDateTime::parse_from_str(&request.from, "%Y-%m-%dT%H:%M:%S").unwrap();
    let to = chrono::NaiveDateTime::parse_from_str(&request.to, "%Y-%m-%dT%H:%M:%S").unwrap();

    let filters = Filters {
        from: Some(from),
        to: Some(to),
        organizer_key: Some(request.organizer_key.clone()),
        status: None,
        event_type: None,
        only_active: Some(true),
    };

    let events = Event::find_events(conn, &filters);
    let closures = Closure::find_by_organizer_key(conn, request.organizer_key.as_str());

    let timeline = Timeline::new(events.clone(), closures);

    let timeline_events = timeline.included(from, Some(to), true);

    Ok(GetTimelineResponse {
        events: timeline_events.into_iter().map(|e| e.into()).collect(),
    })
}