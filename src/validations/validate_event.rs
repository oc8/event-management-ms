use std::str::FromStr;
use tonic::{Code, Status};
use protos::booking::v1::{CancelEventRequest, CreateEventRequest, DeleteEventRequest, EventType, GetEventRequest, ListEventsRequest, UpdateEventRequest};
use crate::errors;
use chrono_tz::Tz;
use rrule::{RRuleSet};
use rrule::ParseError::MissingStartDate;
use uuid::Uuid;
use validator::{ValidateRange};
use crate::errors::{format_error, format_errors};
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
        errors.push(errors::INVALID_EVENT_NAME)
    }

    let start = chrono::NaiveDateTime::parse_from_str(&req.start, "%Y-%m-%dT%H:%M:%S");
    if start.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    let end = chrono::NaiveDateTime::parse_from_str(&req.end, "%Y-%m-%dT%H:%M:%S");
    if end.is_err() {
        errors.push(errors::INVALID_DATETIME)
    }

    if start.is_ok() && end.is_ok() && (start.unwrap() >= end.unwrap()) {
        errors.push(errors::INVALID_DATE_RANGE)
    }

    if !req.timezone.is_empty() && Tz::from_str(req.timezone.as_str()).is_err() {
        errors.push(errors::INVALID_TIMEZONE)
    }

    if !validate_recurrence_rule(req.recurrence_rule.as_str()) {
        errors.push(errors::INVALID_RECURRENCE_RULE)
    }

    match EventType::try_from(req.event_type) {
        Ok(et) => {
            match et {
                EventType::Unspecified => errors.push(errors::INVALID_EVENT_TYPE),
                EventType::Meeting => {
                    if
                        !req.capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000)) &&
                        !req.slot_capacity.validate_range(Option::from(1), Option::from(10000), Option::from(1), Option::from(10000))
                    {
                        errors.push(errors::INVALID_CAPACITY)
                    }

                    if !req.slot_duration.validate_range(Option::from(1), Option::from(1440), Option::from(1), Option::from(1440)) {
                        errors.push(errors::INVALID_SLOT_DURATION)
                    }
                },
                _ => ()
            }
        },
        Err(_) => errors.push(errors::INVALID_EVENT_TYPE)
    }

    if req.organizer_key.is_empty() {
        errors.push(errors::INVALID_ORGANIZER_KEY)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_get_event_request(req: &GetEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(errors::INVALID_EVENT_ID)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_update_event_request(req: &UpdateEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(errors::INVALID_EVENT_ID)
    }

    if !req.start.is_empty() {
        let start = chrono::NaiveDateTime::parse_from_str(&req.start, "%Y-%m-%dT%H:%M:%S");
        if start.is_err() {
            errors.push(errors::INVALID_DATETIME)
        }
    }

    if !req.end.is_empty() {
        let end = chrono::NaiveDateTime::parse_from_str(&req.end, "%Y-%m-%dT%H:%M:%S");
        if end.is_err() {
            errors.push(errors::INVALID_DATETIME)
        }
    }

    if !req.timezone.is_empty() && Tz::from_str(&req.timezone).is_err() {
        errors.push(errors::INVALID_TIMEZONE)
    }

    if !req.recurrence_rule.is_empty() && !validate_recurrence_rule(&req.recurrence_rule) {
        errors.push(errors::INVALID_RECURRENCE_RULE)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_delete_event_request(req: &DeleteEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(errors::INVALID_EVENT_ID)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_list_events_request(req: &ListEventsRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if req.filters.is_none() {
        return Err(format_error(errors::INVALID_FILTERS))
    }

    match validate_date_filters(&req.filters) {
        Ok(_) => (),
        Err(mut e) => errors.append(&mut e)
    }

    if req.filters.as_ref().unwrap().organizer_key.is_empty() {
        errors.push(errors::INVALID_ORGANIZER_KEY)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}

pub fn validate_cancel_event_request(req: &CancelEventRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    if Uuid::parse_str(&req.id).is_err() {
        errors.push(errors::INVALID_EVENT_ID)
    }

    if !errors.is_empty() {
        return Err(format_errors(Code::InvalidArgument, errors))
    }

    Ok(())
}
