use std::str::FromStr;
use chrono::DateTime;
use tonic::{Status};
use protos::booking::v1::{CancelEventRequest, CreateEventRequest, DeleteEventRequest, EventType, GetEventRequest, GetTimelineRequest, UpdateEventRequest};
use crate::errors;
use chrono_tz::Tz;
use rrule::{RRuleSet};
use rrule::ParseError::MissingStartDate;
use uuid::Uuid;
use validator::{ValidateLength, ValidateRange};
use crate::errors::{format_validation_errors, validation_error};
use crate::validations::validate_date_filters;

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

pub fn validate_create_event_request(req: &CreateEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();
    if req.name.is_empty() {
        errors.push(validation_error(vec!["name"], "name is required", errors::ValidationErrorCode::Required))
    }

    let start = DateTime::parse_from_rfc3339(&req.start)
        .map_err(|_| errors.push(validation_error(vec!["start"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    let end = DateTime::parse_from_rfc3339(&req.end)
        .map_err(|_| errors.push(validation_error(vec!["end"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));

    if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
        errors.push(validation_error(vec!["start", "end"], "start must be before end", errors::ValidationErrorCode::InvalidRange))
    }

    if !req.timezone.is_empty() && Tz::from_str(req.timezone.as_str()).is_err() {
        errors.push(validation_error(vec!["timezone"], "invalid timezone", errors::ValidationErrorCode::InvalidTimezone))
    }

    if !validate_recurrence_rule(req.recurrence_rule.as_str()) {
        errors.push(validation_error(vec!["recurrence_rule"], "invalid recurrence rule", errors::ValidationErrorCode::InvalidRecurrenceRule))
    }

    match EventType::try_from(req.event_type) {
        Ok(et) => {
            match et {
                EventType::Unspecified => errors.push(validation_error(vec!["event_type"], "invalid event type", errors::ValidationErrorCode::InvalidEventType)),
                EventType::Meeting => {
                    if
                        !req.capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000)) &&
                        !req.slot_capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000))
                    {
                        errors.push(validation_error(vec!["capacity", "slot_capacity"], "capacity or slot_capacity must be between 1 and 10000", errors::ValidationErrorCode::InvalidRange))
                    }

                    if !req.slot_duration.validate_range(Option::from(1), Option::from(1440), Option::from(1), Option::from(1440)) {
                        errors.push(validation_error(vec!["slot_duration"], "slot_duration must be between 1 and 1440", errors::ValidationErrorCode::InvalidRange))
                    }
                },
                _ => ()
            }
        },
        Err(_) => errors.push(validation_error(vec!["event_type"], "invalid event type", errors::ValidationErrorCode::InvalidEventType))
    }

    if !req.organizer_key.validate_length(Some(1), Some(100), None) {
        errors.push(validation_error(vec!["organizer_key"], "organizer_key must be between 1 and 100 characters", errors::ValidationErrorCode::InvalidLength))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_get_event_request(req: &GetEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(validation_error(vec!["id"], "invalid event id", errors::ValidationErrorCode::InvalidId))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_update_event_request(req: &UpdateEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(validation_error(vec!["id"], "invalid event id", errors::ValidationErrorCode::InvalidId));
    }

    if !req.start.is_empty() {
        let _ = DateTime::parse_from_rfc3339(&req.start)
            .map_err(|_| errors.push(validation_error(vec!["start"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));
    }

    if !req.end.is_empty() {
        let _ = DateTime::parse_from_rfc3339(&req.end)
            .map_err(|_| errors.push(validation_error(vec!["end"], "invalid datetime", errors::ValidationErrorCode::InvalidRfc3339)));
    }

    if !req.timezone.is_empty() && Tz::from_str(&req.timezone).is_err() {
        errors.push(validation_error(vec!["timezone"], "invalid timezone", errors::ValidationErrorCode::InvalidTimezone))
    }

    if !req.recurrence_rule.is_empty() && !validate_recurrence_rule(&req.recurrence_rule) {
        errors.push(validation_error(vec!["recurrence_rule"], "invalid recurrence rule", errors::ValidationErrorCode::InvalidRecurrenceRule))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_delete_event_request(req: &DeleteEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(validation_error(vec!["id"], "invalid event id", errors::ValidationErrorCode::InvalidId))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_get_timeline_request(req: &GetTimelineRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.filters.is_none() {
        return Err(format_validation_errors(vec![validation_error(vec!["filters"], "filters is required", errors::ValidationErrorCode::Required)]))
    }

    match validate_date_filters(&req.filters) {
        Ok(_) => (),
        Err(mut e) => errors.append(&mut e)
    }

    if !req.filters.as_ref().unwrap().organizer_key.validate_length(Some(1), Some(100), None) {
        errors.push(validation_error(vec!["filters.organizer_key"], "organizer_key must be between 1 and 100 characters", errors::ValidationErrorCode::InvalidLength))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}

pub fn validate_cancel_event_request(req: &CancelEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(validation_error(vec!["id"], "invalid event id", errors::ValidationErrorCode::InvalidId))
    }

    if !errors.is_empty() {
        return Err(format_validation_errors(errors))
    }

    Ok(())
}
