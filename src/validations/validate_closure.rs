use tonic::{Code, Status};
use protos::booking::v1::CreateClosureRequest;
use crate::errors;

pub fn validate_create_closing_exception_request(req: &CreateClosureRequest) -> Result<(), Status> {
    let start = chrono::NaiveDateTime::parse_from_str(&req.closing_from, "%Y-%m-%dT%H:%M:%S");
    if start.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_CLOSING_START_DATE))
    }

    let end = chrono::NaiveDateTime::parse_from_str(&req.closing_to, "%Y-%m-%dT%H:%M:%S");
    if end.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_CLOSING_END_DATE))
    }

    if start.unwrap() >= end.unwrap() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_CLOSING_DATE_RANGE))
    }

    if req.organizer_key.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_ORGANIZER_KEY))
    }

    Ok(())
}