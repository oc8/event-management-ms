use chrono::DateTime;
use tonic::Status;
use uuid::Uuid;
use protos::event::v1::{CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse, ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::server::services::v1::closure::closure_model::{ClosureInsert, ClosureRepository, ClosureUpdate};
use crate::utils::filters::{ClosureFilters, Filters};

pub async fn create_closure(
    request: CreateClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<CreateClosureResponse, Status> {
    // validate_create_closure_request(&request)?;

    let closing_from = DateTime::parse_from_rfc3339(&request.closing_from)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();
    let closing_to = DateTime::parse_from_rfc3339(&request.closing_to)
        .map_err(|_| format_error(errors::INVALID_DATETIME))?
        .naive_utc();

    let closure = conn.create_closure(&ClosureInsert{
        closing_from,
        closing_to,
        organizer_key: request.organizer_key,
    })
        .await?;

    Ok(CreateClosureResponse{
        closure: Some(closure.into())
    })
}

pub async fn list_closures(
    request: ListClosuresRequest,
    conn: &mut PgPooledConnection
) -> Result<ListClosuresResponse, Status> {
    // validate_list_closures_request(&request)?;

    let filters: Filters<ClosureFilters> = request.filters.into();

    let closures = conn.get_closures_with_filters(&filters)
        .await?;

    Ok(ListClosuresResponse{
        closures: closures.into_iter().map(|closure| closure.into()).collect()
    })
}

pub async fn update_closure(
    request: UpdateClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<UpdateClosureResponse, Status> {
    // validate_update_closure_request(&request)?;

    let closure_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    let closing_from = match request.closing_from.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.closing_from)
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
            .naive_utc())
    };
    let closing_to = match request.closing_to.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.closing_to)
            .map_err(|_| format_error(errors::INVALID_DATETIME))?
            .naive_utc())
    };

    let updated_closure = conn.update_closure(closure_id, &ClosureUpdate{
        closing_from,
        closing_to,
    }).await?;

    Ok(UpdateClosureResponse{
        closure: Some(updated_closure.into())
    })
}

pub async fn delete_closure(
    request: DeleteClosureRequest,
    conn: &mut PgPooledConnection
) -> Result<DeleteClosureResponse, Status> {
    // validate_delete_closure_request(&request)?;

    let closure_id = Uuid::parse_str(&request.id)
        .map_err(|_| format_error(errors::INVALID_ID))?;

    conn.delete_closure(closure_id)
        .await?;

    Ok(DeleteClosureResponse{
        message: "Closure deleted successfully".to_string()
    })
}
