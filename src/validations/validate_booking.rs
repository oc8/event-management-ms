use tonic::{Code, Status};
use uuid::Uuid;
use validator::{ValidateLength, ValidateRange};
use protos::booking::v1::{CreateBookingRequest, DeleteBookingRequest, GetBookingRequest, ListBookingsRequest};
use crate::errors;
use crate::errors::{format_error, format_errors};

pub fn validate_create_booking_request(req: &CreateBookingRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if !req.booking_holder_key.validate_length(Some(1), Some(100), None) {
        errors.push(errors::INVALID_BOOKING_HOLDER_KEY)
    }

    if Uuid::parse_str(&req.slot_id).is_err() {
        errors.push(errors::INVALID_SLOT_ID)
    }

    let date_time = chrono::NaiveDateTime::parse_from_str(&req.date_time, "%Y-%m-%dT%H:%M:%S");
    if date_time.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    if !req.persons.validate_range(Some(0), Some(10000), Some(0), Some(10000)) {
        errors.push(errors::INVALID_PERSONS_NUMBER)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_get_booking_request(req: &GetBookingRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_error(errors::INVALID_BOOKING_ID))
    }

    Ok(())
}

pub fn validate_delete_booking_request(req: &DeleteBookingRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_error(errors::INVALID_BOOKING_ID))
    }

    Ok(())
}

pub fn validate_list_bookings_request(req: &ListBookingsRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.filters.is_none() {
        return Err(format_error(errors::INVALID_FILTERS))
    }

    if !req.filters.as_ref().unwrap().organizer_key.validate_length(Some(1), Some(100), None) {
        errors.push(errors::INVALID_ORGANIZER_KEY)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}