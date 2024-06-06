use tonic::Status;
use protos::booking::v1::{GetTimelineRequest, GetTimelineResponse};
use crate::database::PgPooledConnection;
use crate::models::booking::{Booking, BookingWithSlot};
use crate::models::closure::Closure;
use crate::models::event::Event;
use crate::models::filters::{BookingFilters, EventFilters, Filters};
use crate::models::timeline::Timeline;
use crate::validations::validate_get_timeline_request;

pub fn get_timeline(
    request: GetTimelineRequest,
    conn: &mut PgPooledConnection
) -> Result<GetTimelineResponse, Status> {
    validate_get_timeline_request(&request)?;

    let filters: Filters<EventFilters> = request.filters.into();

    let mut events = Event::find_events(conn, &filters);

    let mut closures: Vec<Closure> = vec![];
    let mut bookings: Vec<BookingWithSlot> = vec![];
    if let Some(organizer_key) = &filters.organizer_key {
        closures = Closure::find_by_organizer_key(conn, organizer_key);
        let booking_filters: Filters<BookingFilters> = Filters::new(filters.from, filters.to, filters.organizer_key, Default::default());
        bookings = Booking::find(conn, &booking_filters);
    }

    let mut timeline = Timeline::new(events.clone(), closures.clone(), bookings);

    events = timeline.included(filters.from.unwrap(), filters.to.unwrap());

    Ok(GetTimelineResponse{
        events: events.into_iter().map(|e| e.into()).collect()
    })
}
