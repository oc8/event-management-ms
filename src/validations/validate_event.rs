use std::str::FromStr;
use tonic::{Code, Status};
use protos::booking::v1::{CreateEventRequest, EventType, GetActiveEventsRequest, GetEventInstancesRequest, GetEventRequest};
use crate::errors;
use chrono_tz::Tz;
use rrule::{RRuleSet};
use rrule::ParseError::MissingStartDate;
use uuid::Uuid;
use validator::{ValidateRange};

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
    if req.name.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_NAME))
    }

    let start = chrono::NaiveDateTime::parse_from_str(&req.start, "%Y-%m-%dT%H:%M:%S");
    if start.is_err() {
        println!("Invalid start date {}", req.start);
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_START_DATE))
    }

    // TODO: limit the event to one day
    let end = chrono::NaiveDateTime::parse_from_str(&req.end, "%Y-%m-%dT%H:%M:%S");
    if end.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_END_DATE))
    }

    if start.unwrap() >= end.unwrap() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_DATE_RANGE))
    }

    if !req.timezone.is_empty() && Tz::from_str(req.timezone.as_str()).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_TIMEZONE))
    }

    if !validate_recurrence_rule(req.recurrence_rule.as_str()) {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_RECURRENCE_RULE))
    }

    if !req.max_guests.validate_range(Option::from(0), Option::from(1000), Option::from(0), Option::from(1000)) {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_MAX_GUESTS))
    }

    match EventType::try_from(req.event_type) {
        Ok(_) => {},
        Err(_) => return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_TYPE))
    }

    if req.organizer_key.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_ORGANIZER_KEY))
    }

    Ok(())
}

pub fn validate_get_event_request(req: &GetEventRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.id).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_ID))
    }

    Ok(())
}

pub fn validate_get_active_events(req: &GetActiveEventsRequest) -> Result<(), Status> {
    if req.filters.is_none() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_FILTERS))
    }

    if req.filters.as_ref().unwrap().organizer_key.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_ORGANIZER_KEY))
    }

    Ok(())
}

pub fn validate_get_event_instances(req: &GetEventInstancesRequest) -> Result<(), Status> {
    if Uuid::parse_str(&req.event_id).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_ID))
    }

    Ok(())
}