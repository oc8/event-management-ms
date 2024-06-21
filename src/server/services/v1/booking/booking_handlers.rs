use chrono::DateTime;
use uuid::Uuid;
use protos::event::v1::{CreateBookingRequest, CreateBookingResponse, DeleteBookingRequest, DeleteBookingResponse, GetBookingRequest, GetBookingResponse, ListBookingsRequest, ListBookingsResponse};
use crate::add_time_to_datetime;
use crate::database::PgPooledConnection;
use crate::errors::{ApiError};
use crate::server::services::v1::booking::booking_model::{BookingInsert, BookingRepository};
use crate::server::services::v1::event::event_model::EventActions;
use crate::server::services::v1::slot::slot_model::SlotRepository;
use crate::utils::filters::{BookingFilters, Filters};
use crate::utils::validation::ValidateRequest;

pub async fn create_booking(
    request: CreateBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateBookingResponse, ApiError> {
    request.validate()?;

    let slot_id = Uuid::parse_str(&request.slot_id)?;

    let slot = conn.get_slot_by_id(slot_id).await?;

    let date_time = DateTime::parse_from_rfc3339(&request.date_time)?
        .naive_utc();

    if date_time < chrono::Utc::now().naive_utc() {
        return Err(ApiError::InvalidRequest("booking date time cannot be in the past".to_string()))
    }

    let event = slot.event.unwrap();

    let available_dates = event.get_available_dates(date_time, 1)?;

    log::debug!("available dates: {:?}", available_dates);

    if
    date_time.time() != slot.start_time ||
        (event.recurrence_rule.is_some() && !available_dates.contains(&date_time.date()))
    {
        return Err(ApiError::InvalidRequest("booking date time does not match slot datetime".to_string()))
    }

    match event.capacity {
        // Check capacity by event
        Some(capacity) => {
            let start_date = add_time_to_datetime(date_time, event.start_time.time());
            let end_date = add_time_to_datetime(date_time, event.end_time.time());

            let sum_persons = conn.sum_persons_by_event(event.id, start_date, end_date)
                .await?;

            if sum_persons + request.persons > capacity {
                return Err(ApiError::InvalidRequest("event capacity full".to_string()))
            }
        },
        // Check capacity by slot
        None => {
            let sum_persons = conn.sum_persons_by_datetime(slot.id, date_time)
                .await?;

            if sum_persons + request.persons >= slot.capacity {
                return Err(ApiError::InvalidRequest("slot capacity full".to_string()))
            }
        }
    }

    let booking = conn.get_booking_holder_booking(slot_id, request.booking_holder_key.clone(), date_time)
        .await;

    match booking {
        Ok(_) => return Err(ApiError::InvalidRequest("booking holder already has a booking for this slot".to_string())),
        Err(e) => {
            Err(e)
        }
    }?;

    let booking = conn.create_booking(&BookingInsert{
        slot_id,
        booking_holder_key: request.booking_holder_key,
        organizer_key: event.organizer_key,
        date_time,
        persons: request.persons,
    }).await?;

    Ok(CreateBookingResponse{
        booking: Some(booking.into())
    })
}

pub async fn get_booking_by_id(
    request: GetBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<GetBookingResponse, ApiError> {
    request.validate()?;

    let booking_id = Uuid::parse_str(&request.id)?;

    let booking = conn.get_booking_by_id(booking_id)
        .await?;

    Ok(GetBookingResponse{
        booking: Some(booking.into())
    })
}

pub async fn list_bookings(
    request: ListBookingsRequest,
    conn: &mut PgPooledConnection
) -> Result<ListBookingsResponse, ApiError> {
    request.validate()?;

    let filters: Filters<BookingFilters> = request.filters.into();

    let bookings = conn.get_bookings_with_filters(&filters)
        .await?;

    Ok(ListBookingsResponse{
        bookings: bookings.into_iter().map(|booking| booking.into()).collect()
    })
}

pub async fn delete_booking(
    request: DeleteBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteBookingResponse, ApiError> {
    request.validate()?;

    let booking_id = Uuid::parse_str(&request.id)?;

    conn.delete_booking(booking_id)
        .await?;

    Ok(DeleteBookingResponse{
        message: "Booking successfully deleted".to_string()
    })
}
