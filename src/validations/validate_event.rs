use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use tonic::{Code, Status};
use protos::booking::v1::{CreateEventRequest};
use crate::errors;
use chrono_tz::Tz;
use rrule::{RRuleSet};
use rrule::ParseError::MissingStartDate;

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

    let start = time::OffsetDateTime::parse(req.start.as_str(), &Rfc3339);
    if start.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_START_DATE))
    }

    let end = time::OffsetDateTime::parse(req.end.as_str(), &Rfc3339);
    if end.is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_END_DATE))
    }

    if start.unwrap() >= end.unwrap() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_EVENT_DATE_RANGE))
    }

    if Tz::from_str(req.timezone.as_str()).is_err() {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_TIMEZONE))
    }

    if !validate_recurrence_rule(req.recurrence_rule.as_str()) {
        return Err(Status::new(Code::InvalidArgument, errors::INVALID_RECURRENCE_RULE))
    }

    Ok(())
}
