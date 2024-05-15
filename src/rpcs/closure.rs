use tonic::Status;
use protos::booking::v1::{CreateClosureRequest, CreateClosureResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::closure::{Closure, NewClosure};
use crate::validations::validate_create_closing_exception_request;

pub fn create_closure(
    request: CreateClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateClosureResponse, Status> {
    validate_create_closing_exception_request(&request)?;

    let closing_from = chrono::NaiveDateTime::parse_from_str(&request.closing_from, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_CLOSING_START_DATE))?;
    let closing_to = chrono::NaiveDateTime::parse_from_str(&request.closing_to, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| format_error(errors::INVALID_CLOSING_END_DATE))?;

    let new_exception = NewClosure {
        closing_from: &closing_from,
        closing_to: &closing_to,
        organizer_key: &request.organizer_key,
        reason: Some(&request.reason),
    };

    let closure = Closure::create(conn, new_exception)
        .map_err(|_| Status::internal("Failed to create closure"))?;

    Ok(CreateClosureResponse{
        closure: Some(closure.into())
    })
}