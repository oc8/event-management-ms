use tonic::Status;
use protos::booking::v1::{CreateClosingExceptionRequest, CreateClosingExceptionResponse};
use crate::database::PgPooledConnection;
use crate::errors::errors;
use crate::models::closing_exception::{ClosingException, NewClosingException};
use crate::validations::validate_create_closing_exception_request;

pub fn create_closing_exception(
    request: CreateClosingExceptionRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateClosingExceptionResponse, Status> {
    validate_create_closing_exception_request(&request)?;

    let closing_from = chrono::NaiveDateTime::parse_from_str(&request.closing_from, "%Y-%m-%dT%H:%M")
        .map_err(|_| Status::invalid_argument(errors::INVALID_CLOSING_START_DATE))?;
    let closing_to = chrono::NaiveDateTime::parse_from_str(&request.closing_to, "%Y-%m-%dT%H:%M")
        .map_err(|_| Status::invalid_argument(errors::INVALID_CLOSING_END_DATE))?;

    let new_exception = NewClosingException {
        closing_from: &closing_from,
        closing_to: &closing_to,
        reason: Some(&request.reason),
    };

    let closing_exception = ClosingException::create(conn, new_exception)
        .map_err(|_| Status::internal("Failed to create closing exception"))?;

    Ok(CreateClosingExceptionResponse{
        exception: Some(closing_exception.into())
    })
}