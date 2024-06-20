use chrono::DateTime;
use tonic::Status;
use uuid::Uuid;
use protos::event::v1::{CreateBookingRequest, CreateBookingResponse, DeleteBookingRequest, DeleteBookingResponse, GetBookingRequest, GetBookingResponse, ListBookingsRequest, ListBookingsResponse};
use crate::add_time_to_datetime;
use crate::database::PgPooledConnection;
use crate::errors::{ApiError, errors, format_error};
use crate::server::services::v1::booking::booking_model::{BookingInsert, BookingRepository};
use crate::server::services::v1::event::event_model::EventActions;
use crate::server::services::v1::slot::slot_model::SlotRepository;
use crate::utils::filters::{BookingFilters, Filters};

pub async fn create_booking(
    request: CreateBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateBookingResponse, ApiError> {
    // validate_create_booking_request(&request)?;

    let slot_id = Uuid::parse_str(&request.slot_id)?;

    let slot = conn.get_slot_by_id(slot_id).await?;

    let date_time = DateTime::parse_from_rfc3339(&request.date_time)?
        .naive_utc();

    if date_time < chrono::Utc::now().naive_utc() {
        return Err(errors::BOOKING_DATE_IN_PAST)
    }

    let event = slot.event.unwrap();

    let available_dates = event.get_available_dates(date_time, 1)?;

    log::debug!("available dates: {:?}", available_dates);

    if
    date_time.time() != slot.start_time ||
        (event.recurrence_rule.is_some() && !available_dates.contains(&date_time.date()))
    {
        return Err(errors::BOOKING_DATE_TIME_MISMATCH)
    }

    match event.capacity {
        // Check capacity by event
        Some(capacity) => {
            let start_date = add_time_to_datetime(date_time, event.start_time.time());
            let end_date = add_time_to_datetime(date_time, event.end_time.time());

            let sum_persons = conn.sum_persons_by_event(event.id, start_date, end_date)
                .await?;

            if sum_persons + request.persons > capacity {
                return Err(errors::BOOKING_CAPACITY_FULL)
            }
        },
        // Check capacity by slot
        None => {
            let sum_persons = conn.sum_persons_by_datetime(slot.id, date_time)
                .await?;

            if sum_persons + request.persons >= slot.capacity {
                return Err(errors::BOOKING_CAPACITY_FULL)
            }
        }
    }

    let booking = conn.get_booking_holder_booking(slot_id, request.booking_holder_key.clone(), date_time)
        .await;

    match booking {
        Ok(_) => return Err(errors::BOOKING_ALREADY_EXISTS),
        Err(_) => {
            // TODO: return error if another error occurs
        }
    }

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
) -> Result<GetBookingResponse, Status> {
    // validate_get_booking_request(&request)?;

    let booking_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    let booking = conn.get_booking_by_id(booking_id)
        .await?;

    Ok(GetBookingResponse{
        booking: Some(booking.into())
    })
}

pub async fn list_bookings(
    request: ListBookingsRequest,
    conn: &mut PgPooledConnection
) -> Result<ListBookingsResponse, Status> {
    // validate_list_bookings_request(&request)?;

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
) -> Result<DeleteBookingResponse, Status> {
    // validate_delete_booking_request(&request)?;

    let booking_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    conn.delete_booking(booking_id)
        .await?;

    Ok(DeleteBookingResponse{
        message: "Booking successfully deleted".to_string()
    })
}
