use tonic::{Code, Status};
use protos::booking::v1::CreateClosureRequest;
use crate::errors;
use crate::errors::{format_errors};

pub fn validate_create_closing_exception_request(req: &CreateClosureRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    let start = chrono::NaiveDateTime::parse_from_str(&req.closing_from, "%Y-%m-%dT%H:%M:%S");
    if start.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    let end = chrono::NaiveDateTime::parse_from_str(&req.closing_to, "%Y-%m-%dT%H:%M:%S");
    if end.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
        errors.push(errors::INVALID_DATE_RANGE)
    }

    if req.organizer_key.is_empty() {
        errors.push(errors::INVALID_ORGANIZER_KEY)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}