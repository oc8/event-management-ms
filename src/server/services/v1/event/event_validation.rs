use std::str::FromStr;
use chrono::DateTime;
use chrono_tz::Tz;
use rrule::ParseError::MissingStartDate;
use rrule::RRuleSet;
use uuid::Uuid;
use validator::{ValidateLength, ValidateRange};
use protos::event::v1::{CancelEventRequest, CreateEventRequest, DeleteEventRequest, EventType, GetEventRequest, GetTimelineRequest, ListEventsRequest, UpdateEventRequest};
use crate::errors::{ApiError, List, ValidationErrorKind, ValidationErrorMessage};
use crate::errors::ApiError::ValidationError;
use crate::utils::filters::validate_date_filters;
use crate::utils::validation::ValidateRequest;

fn validate_recurrence_rule(rule: &str) -> bool {
    if rule.is_empty() {
        return true;
    }

    match rule.parse::<RRuleSet>() {
        Ok(_) => true,
        Err(e) => {
            if e == rrule::RRuleError::ParserError(MissingStartDate) {
                return true
            }
            false
        },
    }
}

impl ValidateRequest for CreateEventRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = Vec::new();

        if !self.name.validate_length(Some(1), Some(100), None) {
            errors.push(ValidationErrorKind::InvalidLength("name".to_string(), 1, 100))
        }

        let start = DateTime::parse_from_rfc3339(&self.start)
            .map_err(|_| {
                errors.push(ValidationErrorKind::InvalidFormat("start".to_string(), ValidationErrorMessage::InvalidDateTime()))
            });

        let end = DateTime::parse_from_rfc3339(&self.end)
            .map_err(|_| {
                errors.push(ValidationErrorKind::InvalidFormat("end".to_string(), ValidationErrorMessage::InvalidDateTime()))
            });

        if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
            errors.push(ValidationErrorKind::InvalidDateRange("start, end".to_string(), "start must be before end".to_string()))
        }

        if !self.timezone.is_empty() && Tz::from_str(self.timezone.as_str()).is_err() {
            errors.push(ValidationErrorKind::InvalidFormat("timezone".to_string(), ValidationErrorMessage::InvalidTimezone()))
        }

        if !validate_recurrence_rule(self.recurrence_rule.as_str()) {
            errors.push(ValidationErrorKind::InvalidFormat("recurrence_rule".to_string(), ValidationErrorMessage::InvalidRecurrenceRule()))
        }

        match EventType::try_from(self.event_type) {
            Ok(et) => {
                match et {
                    EventType::Unspecified => errors.push(ValidationErrorKind::InvalidType("event_type".to_string(), EventType::Unspecified.as_str_name().to_string())),
                    EventType::Meeting => {
                        if
                            !self.capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000)) &&
                            !self.slot_capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000))
                        {
                            errors.push(ValidationErrorKind::InvalidRange("capacity, slot_capacity".to_string(), 1, 10000))
                        }

                        if !self.slot_duration.validate_range(Option::from(1), Option::from(1440), Option::from(1), Option::from(1440)) {
                            errors.push(ValidationErrorKind::InvalidRange("slot_duration".to_string(), 1, 1440))
                        }
                    },
                    _ => ()
                }
            },
            Err(_) => errors.push(ValidationErrorKind::InvalidType("event_type".to_string(), "".to_string()))
        }

        if !self.organizer_key.validate_length(Some(1), Some(100), None) {
            errors.push(ValidationErrorKind::InvalidLength("organizer_key".to_string(), 1, 100))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)))
        }

        Ok(())
    }
}

impl ValidateRequest for GetEventRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid())])))
        }

        Ok(())
    }
}

impl ValidateRequest for ListEventsRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = Vec::new();

        if self.filters.is_none() {
            return Err(ValidationError(List(vec![ValidationErrorKind::MissingField("filters".to_string())])))
        }

        match validate_date_filters(&self.filters) {
            Ok(_) => (),
            Err(mut e) => errors.append(&mut e)
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

impl ValidateRequest for UpdateEventRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = Vec::new();

        if Uuid::parse_str(&self.id).is_err() {
            errors.push(ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid()))
        }

        if !self.start.is_empty() {
            let _ = DateTime::parse_from_rfc3339(&self.start)
                .map_err(|_| {
                    errors.push(ValidationErrorKind::InvalidFormat("start".to_string(), ValidationErrorMessage::InvalidDateTime()))
                });
        }

        // TODO: add a database constraint to ensure that the end date is always greater than the start date
        if !self.end.is_empty() {
            let _ = DateTime::parse_from_rfc3339(&self.end)
                .map_err(|_| {
                    errors.push(ValidationErrorKind::InvalidFormat("end".to_string(), ValidationErrorMessage::InvalidDateTime()))
                });
        }

        if !self.timezone.is_empty() && Tz::from_str(&self.timezone).is_err() {
            errors.push(ValidationErrorKind::InvalidFormat("timezone".to_string(), ValidationErrorMessage::InvalidTimezone()))
        }

        if !self.recurrence_rule.is_empty() && !validate_recurrence_rule(&self.recurrence_rule) {
            errors.push(ValidationErrorKind::InvalidFormat("recurrence_rule".to_string(), ValidationErrorMessage::InvalidRecurrenceRule()))
        }

        if !errors.is_empty() {
            return Err(ValidationError(List::<ValidationErrorKind>(errors)))
        }

        Ok(())
    }
}

impl ValidateRequest for DeleteEventRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid())])))
        }

        Ok(())
    }
}

impl ValidateRequest for CancelEventRequest {
    fn validate(&self) -> Result<(), ApiError> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidFormat("id".to_string(), ValidationErrorMessage::InvalidUuid())])))
        }

        if !self.reason.is_empty() && !self.reason.validate_length(Some(1), Some(500), None) {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidLength("reason".to_string(), 1, 500)])))
        }

        if !self.reason.is_empty() && !self.canceled_by.validate_length(Some(1), Some(100), None) {
            return Err(ValidationError(List(vec![ValidationErrorKind::InvalidLength("canceled_by".to_string(), 1, 100)])))
        }

        Ok(())
    }
}

impl ValidateRequest for GetTimelineRequest {
    fn validate(&self) -> Result<(), ApiError> {
        let mut errors = Vec::new();

        if self.filters.is_none() {
            return Err(ValidationError(List(vec![ValidationErrorKind::MissingField("filters".to_string())])))
        }

        match validate_date_filters(&self.filters) {
            Ok(_) => (),
            Err(mut e) => errors.append(&mut e)
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