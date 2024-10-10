use std::fmt;
use redis::RedisError;
use rrule::RRuleError;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use thiserror::Error;
use tonic_error::TonicError;
use crate::report_error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ValidationErrorMessage {
    #[error("must be a valid UUID")]
    InvalidUuid(),
    #[error("must be a valid rfc3339 datetime")]
    InvalidDateTime(),
    #[error("must be a timezone (e.g. Europe/Paris)")]
    InvalidTimezone(),
    #[error("must be a valid recurrence rule (e.g. FREQ=WEEKLY;BYDAY=TH)")]
    InvalidRecurrenceRule(),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ValidationErrorKind {
    #[error("{0} must be between {1} and {2}, field: {0}")]
    InvalidLength(String, usize, usize),
    #[error("{0} {1}, field: {0}")]
    InvalidFormat(String, ValidationErrorMessage),
    #[error("{0} must be between {1} and {2}, field: {0}")]
    InvalidRange(String, usize, usize),
    #[error("{1}, field: {0}")]
    InvalidDateRange(String, String),
    #[error("{0} is missing, field: {0}")]
    MissingField(String),
    #[error("{1} is an invalid type, field: {0}")]
    InvalidType(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List<T>(pub Vec<T>);

impl<T> fmt::Display for List<T>
    where
        T: fmt::Display + serde::Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&item.to_string());
        }
        write!(f, "{}", s)
    }
}

#[derive(Error, Debug, Serialize, Deserialize, TonicError)]
#[non_exhaustive]
pub enum ApiError {
    #[error("internal server error")]
    InternalServerError,
    #[error("{0}")]
    InvalidRequest(String),
    #[error("redis connection failure")]
    RedisConnectionFailure,
    #[error("cache error")]
    CacheError,
    #[error("database connection failure")]
    DatabaseConnectionFailure,
    #[error("{0}")]
    DatabaseError(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ParsingError(String),
    #[error("booking cannot be in past")]
    BookingInPast,
    #[error("booking datetime does not match slot datetime")]
    InvalidBookingDatetime,
    #[error("event capacity full")]
    EventCapacityFull,
    #[error("slot capacity full")]
    SlotCapacityFull,
    #[error("booking already exists for the same slot")]
    BookingAlreadyExists,
    #[error("validation error: {0}")]
    ValidationError(List<ValidationErrorKind>),
}

impl ApiError {
    pub fn code(&self) -> tonic::Code {
        match self {
            ApiError::CacheError
            | ApiError::RedisConnectionFailure
            | ApiError::DatabaseConnectionFailure => tonic::Code::Unavailable,
            ApiError::DatabaseError(_) => tonic::Code::Internal,
            ApiError::AlreadyExists(_) => tonic::Code::AlreadyExists,
            ApiError::NotFound(_) => tonic::Code::NotFound,
            ApiError::ValidationError(_)
            | ApiError::InvalidBookingDatetime
            | ApiError::BookingInPast
            | ApiError::EventCapacityFull
            | ApiError::SlotCapacityFull
            | ApiError::InvalidRequest(_) => tonic::Code::InvalidArgument,
            _ => tonic::Code::Internal,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            ApiError::ValidationError(_) => true,
            _ => false,
        }
    }

    pub fn errors(&self) -> serde_json::Value {
        match self {
            ApiError::ValidationError(errors) => {
                let mut v = Vec::new();
                for e in &errors.0 {
                    let type_name = to_variant_name(e).unwrap();
                    let data = e.to_string();
                    let (message, field) = data.split_once(", field: ").unwrap_or(("", ""));
                    v.push(serde_json::json!({
                        "message": message,
                        "field": field,
                        "type": type_name,
                    }));
                }

                serde_json::json!(v)
            }
            _ => serde_json::json!([])
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => {
                // TODO: find a way to return the table name
                ApiError::NotFound("".to_string())
            },
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    ApiError::AlreadyExists(e.message().to_string())
                } else {
                    report_error(&e);
                    ApiError::DatabaseError(e.message().to_string())
                }
            }
            _ => {
                report_error(&error);
                ApiError::InternalServerError
            }
        }
    }
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> Self {
        report_error(&error);
        ApiError::CacheError
    }
}

impl From<chrono::ParseError> for ApiError {
    fn from(error: chrono::ParseError) -> Self {
        report_error(&error);
        ApiError::ParsingError("Cannot parse date".to_string())
    }
}

impl From<uuid::Error> for ApiError {
    fn from(error: uuid::Error) -> Self {
        report_error(&error);
        ApiError::ParsingError("Cannot parse uuid".to_string())
    }
}

impl From<RRuleError> for ApiError {
    fn from(error: RRuleError) -> Self {
        report_error(&error);
        ApiError::ParsingError("Cannot parse rrule".to_string())
    }
}

impl PartialEq for ApiError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ApiError::InternalServerError, ApiError::InternalServerError) => true,
            (ApiError::InvalidRequest(a), ApiError::InvalidRequest(b)) => a == b,
            (ApiError::RedisConnectionFailure, ApiError::RedisConnectionFailure) => true,
            (ApiError::CacheError, ApiError::CacheError) => true,
            (ApiError::DatabaseConnectionFailure, ApiError::DatabaseConnectionFailure) => true,
            (ApiError::DatabaseError(a), ApiError::DatabaseError(b)) => a == b,
            (ApiError::AlreadyExists(a), ApiError::AlreadyExists(b)) => a == b,
            (ApiError::NotFound(a), ApiError::NotFound(b)) => a == b,
            (ApiError::ParsingError(a), ApiError::ParsingError(b)) => a == b,
            _ => false,
        }
    }
}
