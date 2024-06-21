use chrono::DateTime;
use uuid::Uuid;
use validator::{ValidateLength, ValidateRange};
use protos::event::v1::{CreateBookingRequest, DeleteBookingRequest, GetBookingRequest, ListBookingsRequest};
use crate::errors::{ApiError, List, ValidationErrorKind, ValidationErrorMessage};
use crate::errors::ApiError::ValidationError;
use crate::utils::validation::{ValidateRequest};

impl ValidateRequest for CreateBookingRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if !self.booking_holder_key.validate_length(Some(1), Some(100), None) {
            errors.push(ValidationErrorKind::InvalidLength("booking_holder_key".to_string(), 1, 100))
        }

        if Uuid::parse_str(&self.slot_id).is_err() {
            errors.push(ValidationErrorKind::InvalidFormat("slot_id".to_string(), ValidationErrorMessage::InvalidUuid()))
        }

        if DateTime::parse_from_rfc3339(&self.date_time).is_err() {
            errors.push(ValidationErrorKind::InvalidFormat("date_time".to_string(), ValidationErrorMessage::InvalidDateTime()))
        }

        if !self.persons.validate_range(Some(0), Some(10000), Some(0), Some(10000)) {
            errors.push(ValidationErrorKind::InvalidRange("persons".to_string(), 0, 10000))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)))
        }

        Ok(())
    }
}

impl ValidateRequest for GetBookingRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid())])))
        }

        Ok(())
    }
}

impl ValidateRequest for ListBookingsRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if self.filters.is_none() {
            return Err(ValidationError(List(vec![ValidationErrorKind::MissingField("filters".to_string())])))
        }

        if !self.filters.as_ref().unwrap().organizer_key.validate_length(Some(1), Some(100), None) {
            errors.push(ValidationErrorKind::InvalidLength("filters.organizer_key".to_string(), 1, 100))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)))
        }

        Ok(())
    }
}

impl ValidateRequest for DeleteBookingRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid())])))
        }

        Ok(())
    }
}
