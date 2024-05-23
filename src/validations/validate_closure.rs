use tonic::{Code, Status};
use uuid::Uuid;
use protos::booking::v1::{CreateClosureRequest, DeleteClosureRequest, ListClosuresRequest, UpdateClosureRequest};
use crate::errors;
use crate::errors::{format_error, format_errors};

pub fn validate_create_closure_request(req: &CreateClosureRequest) -> Result<(), Status> {
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

pub fn validate_update_closure_request(req: &UpdateClosureRequest) -> Result<(), Status> {
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

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_delete_closure_request(req: &DeleteClosureRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_error(errors::INVALID_CLOSURE_ID))
    }

    Ok(())
}

pub fn validate_list_closures_request(req: &ListClosuresRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.filters.is_none() {
        return Err(format_error(errors::INVALID_FILTERS))
    }

    if req.filters.as_ref().unwrap().organizer_key.is_empty() {
        errors.push(errors::INVALID_ORGANIZER_KEY)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}