use chrono::DateTime;
use tonic::Status;
use uuid::Uuid;
use protos::booking::v1::{CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::closure::{Closure, NewClosure};
use crate::models::filters::{ClosureFilters, Filters};
use crate::validations::{validate_create_closure_request, validate_delete_closure_request, validate_list_closures_request, validate_update_closure_request};

pub fn create_closure(
    request: CreateClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateClosureResponse, Status> {
    validate_create_closure_request(&request)?;

    let closing_from = DateTime::parse_from_rfc3339(&request.closing_from)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();
    let closing_to = DateTime::parse_from_rfc3339(&request.closing_to)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();

    let new_exception = NewClosure {
        closing_from: &closing_from,
        closing_to: &closing_to,
        organizer_key: &request.organizer_key,
    };

    let closure = Closure::create(conn, new_exception)
        .map_err(|_| format_error(errors::CLOSURE_CREATION_FAILED))?;

    Ok(CreateClosureResponse{
        closure: Some(closure.into())
    })
}

pub fn update_closure(
    request: UpdateClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<UpdateClosureResponse, Status> {
    validate_update_closure_request(&request)?;

    let closure_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    let closure = Closure::find_by_id(conn, closure_id)
        .ok_or_else(|| format_error(errors::CLOSURE_NOT_FOUND))?;

    let closing_from = DateTime::parse_from_rfc3339(&request.closing_from)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();
    let closing_to = DateTime::parse_from_rfc3339(&request.closing_to)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();

    let updated_closure = Closure::update(conn, closure_id, NewClosure{
        closing_from: &closing_from,
        closing_to: &closing_to,
        organizer_key: &closure.organizer_key,
    })
        .map_err(|_| format_error(errors::CLOSURE_UPDATE_FAILED))?;

    Ok(UpdateClosureResponse{
        closure: Some(updated_closure.into())
    })
}

pub fn delete_closure(
    request: DeleteClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteClosureResponse, Status> {
    validate_delete_closure_request(&request)?;

    let closure_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    Closure::delete(conn, closure_id)
        .map_err(|_| format_error(errors::CLOSURE_DELETION_FAILED))?;

    Ok(DeleteClosureResponse{
        message: "Closure deleted successfully".to_string()
    })
}

pub fn list_closures(
    request: ListClosuresRequest,
    conn: &mut PgPooledConnection
) -> Result<ListClosuresResponse, Status> {
    validate_list_closures_request(&request)?;

    let filters: Filters<ClosureFilters> = request.filters.into();

    let closures = Closure::find(conn, &filters);

    Ok(ListClosuresResponse{
        closures: closures.into_iter().map(|closure| closure.into()).collect()
    })
}