use chrono::DateTime;
use tonic::{Status};
use uuid::Uuid;
use validator::ValidateLength;
use protos::booking::v1::{CreateClosureRequest, DeleteClosureRequest, ListClosuresRequest, UpdateClosureRequest};
use crate::errors;
use crate::errors::{format_validation_errors, validation_error};

pub fn validate_create_closure_request(req: &CreateClosureRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    let start = DateTime::parse_from_rfc3339(&req.closing_from)
        .map_err(|_| errors.push(validation_error(vec!["start"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    let end = DateTime::parse_from_rfc3339(&req.closing_to)
        .map_err(|_| errors.push(validation_error(vec!["end"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
        errors.push(validation_error(vec!["start", "end"], "start must be before end", errors::ValidationErrorCode::InvalidRange))
    }

    if !req.organizer_key.validate_length(Some(1), Some(100), None) {
        errors.push(validation_error(vec!["organizer_key"], "organizer_key must be between 1 and 100 characters", errors::ValidationErrorCode::InvalidLength))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_update_closure_request(req: &UpdateClosureRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_validation_errors(vec![validation_error(vec!["id"], "invalid closure id", errors::ValidationErrorCode::InvalidId)]))
    }

    let start = DateTime::parse_from_rfc3339(&req.closing_from)
        .map_err(|_| errors.push(validation_error(vec!["start"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    let end = DateTime::parse_from_rfc3339(&req.closing_to)
        .map_err(|_| errors.push(validation_error(vec!["end"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
        errors.push(validation_error(vec!["start", "end"], "start must be before end", errors::ValidationErrorCode::InvalidRange))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_delete_closure_request(req: &DeleteClosureRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_validation_errors(vec![validation_error(vec!["id"], "invalid closure id", errors::ValidationErrorCode::InvalidId)]))
    }

    Ok(())
}

pub fn validate_list_closures_request(req: &ListClosuresRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.filters.is_none() {
        return Err(format_validation_errors(vec![validation_error(vec!["filters"], "filters is required", errors::ValidationErrorCode::Required)]))
    }

    if !req.filters.as_ref().unwrap().organizer_key.validate_length(Some(1), Some(100), None) {
        errors.push(validation_error(vec!["filters.organizer_key"], "organizer_key must be between 1 and 100 characters", errors::ValidationErrorCode::InvalidLength))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}