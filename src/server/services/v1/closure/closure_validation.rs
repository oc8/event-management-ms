use crate::errors::ApiError::ValidationError;
use crate::errors::{ApiError, List, ValidationErrorKind, ValidationErrorMessage};
use crate::utils::filters::validate_date_filters;
use crate::utils::validation::ValidateRequest;
use chrono::DateTime;
use event_protos::event::v1::{
    CreateClosureRequest, DeleteClosureRequest, ListClosuresRequest, UpdateClosureRequest,
};
use uuid::Uuid;
use validator::ValidateLength;

impl ValidateRequest for CreateClosureRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        let start = DateTime::parse_from_rfc3339(&self.closing_from).map_err(|_| {
            errors.push(ValidationErrorKind::InvalidFormat(
                "closing_from".to_string(),
                ValidationErrorMessage::InvalidDateTime(),
            ))
        });

        let end = DateTime::parse_from_rfc3339(&self.closing_to).map_err(|_| {
            errors.push(ValidationErrorKind::InvalidFormat(
                "closing_to".to_string(),
                ValidationErrorMessage::InvalidDateTime(),
            ))
        });

        if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
            errors.push(ValidationErrorKind::InvalidDateRange(
                "closing_from, closing_to".to_string(),
                "closing_from must be before closing_to".to_string(),
            ))
        }

        if !self.organizer_key.validate_length(Some(1), Some(100), None) {
            errors.push(ValidationErrorKind::InvalidLength(
                "organizer_key".to_string(),
                1,
                100,
            ))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)));
        }

        Ok(())
    }
}

impl ValidateRequest for ListClosuresRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if self.filters.is_none() {
            return Err(ValidationError(List::<ValidationErrorKind>(vec![
                ValidationErrorKind::MissingField("filters".to_string()),
            ])));
        }

        match validate_date_filters(&self.filters) {
            Ok(_) => (),
            Err(mut e) => errors.append(&mut e),
        }

        if !self
            .filters
            .as_ref()
            .unwrap()
            .organizer_key
            .validate_length(Some(1), Some(100), None)
        {
            errors.push(ValidationErrorKind::InvalidLength(
                "filters.organizer_key".to_string(),
                1,
                100,
            ))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)));
        }

        Ok(())
    }
}

impl ValidateRequest for UpdateClosureRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = vec![];

        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List::<ValidationErrorKind>(vec![
                ValidationErrorKind::InvalidFormat(
                    "id".to_string(),
                    ValidationErrorMessage::InvalidUuid(),
                ),
            ])));
        }

        let start = DateTime::parse_from_rfc3339(&self.closing_from).map_err(|_| {
            errors.push(ValidationErrorKind::InvalidFormat(
                "closing_from".to_string(),
                ValidationErrorMessage::InvalidDateTime(),
            ))
        });

        let end = DateTime::parse_from_rfc3339(&self.closing_to).map_err(|_| {
            errors.push(ValidationErrorKind::InvalidFormat(
                "closing_to".to_string(),
                ValidationErrorMessage::InvalidDateTime(),
            ))
        });

        if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
            errors.push(ValidationErrorKind::InvalidDateRange(
                "closing_from, closing_to".to_string(),
                "closing_from must be before closing_to".to_string(),
            ))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)));
        }

        Ok(())
    }
}

impl ValidateRequest for DeleteClosureRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![
                ValidationErrorKind::InvalidFormat(
                    "id".to_string(),
                    ValidationErrorMessage::InvalidUuid(),
                ),
            ])));
        }
        Ok(())
    }
}
