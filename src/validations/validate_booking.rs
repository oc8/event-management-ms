use tonic::{Code, Status};
use uuid::Uuid;
use validator::{ValidateRange};
use protos::booking::v1::{CreateBookingRequest, GetBookingRequest};
use crate::errors;
use crate::errors::{format_error, format_errors};

pub fn validate_create_booking_request(req: &CreateBookingRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.booking_holder_key.is_empty() {
        errors.push(errors::INVALID_BOOKING_HOLDER_KEY)
    }

    if Uuid::parse_str(&req.slot_id).is_err() {
        errors.push(errors::INVALID_SLOT_ID)
    }

    let date_time = chrono::NaiveDateTime::parse_from_str(&req.date_time, "%Y-%m-%dT%H:%M:%S");
    if date_time.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    if !req.persons.validate_range(Option::from(0), Option::from(10000), Option::from(0), Option::from(10000)) {
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