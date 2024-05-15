use tonic::{Code, Status};
use uuid::Uuid;
use protos::booking::v1::{CreateBookingRequest, GetBookingRequest};
use crate::errors;

pub fn validate_create_booking_request(req: &CreateBookingRequest) -> Result<(), Status> {
    if req.booking_holder_key.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_BOOKING_HOLDER_KEY))
    }

    if Uuid::parse_str(&req.slot_id).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_SLOT_ID))
    }

    let date_time = chrono::NaiveDateTime::parse_from_str(&req.date_time, "%Y-%m-%dT%H:%M:%S");
    if date_time.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_BOOKING_DATE))
    }

    Ok(())
}

pub fn validate_get_booking_request(req: &GetBookingRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_BOOKING_ID))
    }

    Ok(())
}