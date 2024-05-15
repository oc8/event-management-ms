use tonic::Status;
use uuid::Uuid;
use protos::booking::v1::{CreateBookingRequest, CreateBookingResponse, GetBookingRequest, GetBookingResponse};
use crate::database::PgPooledConnection;
use crate::errors::errors;
use crate::models::booking::{Booking, NewBooking};
use crate::models::slot::Slot;
use crate::validations::{validate_create_booking_request, validate_get_booking_request};

pub fn create_booking(
    request: CreateBookingRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateBookingResponse, Status> {
    validate_create_booking_request(&request)?;

    let slot_id = Uuid::parse_str(&request.slot_id).map_err(|_| Status::invalid_argument(errors::INVALID_SLOT_ID))?;
    let slot = Slot::find_by_id(conn, slot_id);

    if slot.is_none() {
        return Err(Status::not_found(errors::SLOT_NOT_FOUND))
    }

    let date_time = chrono::NaiveDateTime::parse_from_str(&request.date_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| Status::invalid_argument(errors::INVALID_BOOKING_DATE))?;

    let new_booking = NewBooking {
        slot_id: &slot_id,
        booking_holder_key: &request.booking_holder_key,
        date_time: &date_time,
    };

    let booking = Booking::create(conn, new_booking)
        .map_err(|_| Status::internal("Failed to create booking"))?;

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
        .map_err(|_| Status::invalid_argument(errors::INVALID_BOOKING_ID))?;

    let booking = Booking::find_by_id(conn, booking_id);

    if booking.is_none() {
        return Err(Status::not_found(errors::BOOKING_NOT_FOUND))
    }

    Ok(GetBookingResponse{
        booking: Some(booking.unwrap().into())
    })
}