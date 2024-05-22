use tonic::{Code, Status};
use uuid::Uuid;
use protos::booking::v1::{GetTimelineRequest};
use crate::errors;
use crate::errors::format_errors;

pub fn validate_get_timeline(req: &GetTimelineRequest) -> Result<(), Status> {
    let mut errors = Vec::new();

    let from = chrono::NaiveDateTime::parse_from_str(&req.from, "%Y-%m-%dT%H:%M:%S");
    if from.is_err() {
        errors.push(errors::INVALID_DATE)
    }

    let to = chrono::NaiveDateTime::parse_from_str(&req.to, "%Y-%m-%dT%H:%M:%S");
    if to.is_err() {
        errors.push(errors::INVALID_DATE)
    }

    if req.from >= req.to {
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