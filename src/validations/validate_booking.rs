use chrono::DateTime;
use tonic::{Status};
use uuid::Uuid;
use validator::{ValidateLength, ValidateRange};
use protos::booking::v1::{CreateBookingRequest, DeleteBookingRequest, GetBookingRequest, ListBookingsRequest};
use crate::errors;
use crate::errors::{format_validation_errors, validation_error};

pub fn validate_create_booking_request(req: &CreateBookingRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if !req.booking_holder_key.validate_length(Some(1), Some(100), None) {
        errors.push(validation_error(vec!["booking_holder_key"], "booking_holder_key must be between 1 and 100 characters", errors::ValidationErrorCode::InvalidLength))
    }

    if Uuid::parse_str(&req.slot_id).is_err() {
        errors.push(validation_error(vec!["id"], "invalid booking id", errors::ValidationErrorCode::InvalidId))
    }

    let _ = DateTime::parse_from_rfc3339(&req.date_time)
        .map_err(|_| errors.push(validation_error(vec!["end"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    if !req.persons.validate_range(Some(0), Some(10000), Some(0), Some(10000)) {
        errors.push(validation_error(vec!["persons"], "persons must be between 0 and 10000", errors::ValidationErrorCode::InvalidRange))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_get_booking_request(req: &GetBookingRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_validation_errors(vec![validation_error(vec!["id"], "invalid booking id", errors::ValidationErrorCode::InvalidId)]))
    }

    Ok(())
}

pub fn validate_delete_booking_request(req: &DeleteBookingRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(format_validation_errors(vec![validation_error(vec!["id"], "invalid booking id", errors::ValidationErrorCode::InvalidId)]))
    }

    Ok(())
}

pub fn validate_list_bookings_request(req: &ListBookingsRequest) -> Result<(), Status> {
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