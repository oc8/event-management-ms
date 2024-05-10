use tonic::{Code, Status};
use uuid::Uuid;
use protos::booking::v1::{CreateBookingRequest};
use crate::errors;

pub fn validate_create_booking_request(req: &CreateBookingRequest) -> Result<(), Status> {
    if req.booking_holder_key.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_BOOKING_HOLDER_KEY))
    }

    if Uuid::parse_str(&req.slot_id).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_SLOT_ID))
    }

    Ok(())
}