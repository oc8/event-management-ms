use log::debug;
use tonic::Status;
use uuid::Uuid;
use booking_ms::add_time_to_datetime;
use protos::booking::v1::{CreateBookingRequest, CreateBookingResponse, DeleteBookingRequest, DeleteBookingResponse, GetBookingRequest, GetBookingResponse, ListBookingsRequest, ListBookingsResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::booking::{Booking, NewBooking};
use crate::models::filters::{BookingFilters, Filters};
use crate::models::slot::Slot;
use crate::validations::{validate_create_booking_request, validate_delete_booking_request, validate_get_booking_request, validate_list_bookings_request};

pub fn create_booking(
    request: CreateBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateBookingResponse, Status> {
    validate_create_booking_request(&request)?;

    let slot_id = Uuid::parse_str(&request.slot_id).map_err(|_| format_error(errors::INVALID_SLOT_ID))?;
    let (slot, event) = Slot::find_by_id_with_event(conn, slot_id)
        .ok_or_else(|| format_error(errors::SLOT_NOT_FOUND))?;

    let date_time = chrono::NaiveDateTime::parse_from_str(&request.date_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_BOOKING_DATE))?;

    if date_time < chrono::Utc::now().naive_utc() {
        return Err(format_error(errors::BOOKING_DATE_IN_PAST))
    }

    let available_dates = event.get_available_dates(date_time, 1)
        .map_err(|_| format_error(errors::INTERNAL))?;

    debug!("available dates: {:?}", available_dates);

    if
        date_time.time() != slot.start_time ||
        (event.recurrence_rule.is_some() && !available_dates.contains(&date_time.date()))
    {
        return Err(format_error(errors::BOOKING_DATE_TIME_MISMATCH))
    }

    match event.capacity {
        // Check capacity by event
        Some(capacity) => {
            let start_date = add_time_to_datetime(date_time, event.start_time.time());
            let end_date = add_time_to_datetime(date_time, event.end_time.time());

            let sum_persons = Booking::sum_persons_by_event(conn, event.id, start_date, end_date)
                .unwrap_or(0);

            if sum_persons + request.persons > capacity {
                return Err(format_error(errors::BOOKING_CAPACITY_FULL))
            }
        },
        // Check capacity by slot
        None => {
            let sum_persons = Booking::sum_persons_by_datetime(conn, slot.id, date_time)
                .unwrap_or(0);

            if sum_persons + request.persons >= slot.capacity {
                return Err(format_error(errors::BOOKING_CAPACITY_FULL))
            }
        }
    }

    let booking = Booking::find_duplicated_booking(conn, slot_id, request.booking_holder_key.clone(), date_time);
    if booking.is_some() {
        return Err(format_error(errors::BOOKING_ALREADY_EXISTS))
    }

    let new_booking = NewBooking {
        slot_id: &slot_id,
        booking_holder_key: &request.booking_holder_key,
        organizer_key: &event.organizer_key,
        date_time: &date_time,
        persons: &request.persons,
    };

    let booking = Booking::create(conn, new_booking)
        .map_err(|_| format_error(errors::BOOKING_CREATION_FAILED))?;

    Ok(CreateBookingResponse{
        booking: Some(booking.into())
    })
}

pub fn get_booking_by_id(
    request: GetBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<GetBookingResponse, Status> {
    validate_get_booking_request(&request)?;

    let booking_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_BOOKING_ID))?;

    let booking = Booking::find_by_id(conn, booking_id)
        .ok_or_else(|| format_error(errors::BOOKING_NOT_FOUND))?;

    Ok(GetBookingResponse{
        booking: Some(booking.into())
    })
}

pub fn delete_booking(
    request: DeleteBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteBookingResponse, Status> {
    validate_delete_booking_request(&request)?;

    let booking_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_BOOKING_ID))?;

    let _ = Booking::find_by_id(conn, booking_id)
        .ok_or_else(|| format_error(errors::BOOKING_NOT_FOUND))?;

    Booking::delete(conn, booking_id)
        .map_err(|_| format_error(errors::BOOKING_DELETION_FAILED))?;

    Ok(DeleteBookingResponse{
        message: "Booking successfully deleted".to_string()
    })
}

pub fn list_bookings(
    request: ListBookingsRequest,
    conn: &mut PgPooledConnection
) -> Result<ListBookingsResponse, Status> {
    validate_list_bookings_request(&request)?;

    let filters: Filters<BookingFilters> = request.filters.into();

    let bookings = Booking::find(conn, &filters);

    Ok(ListBookingsResponse{
        bookings: bookings.into_iter().map(|booking| booking.into()).collect()
    })
}