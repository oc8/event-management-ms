use crate::database::PgPooledConnection;
use crate::errors::ApiError;
use crate::get_meta_timezone;
use crate::server::services::v1::closure::closure_model::{
    ClosureInsert, ClosureRepository, ClosureUpdate,
};
use crate::utils::filters::{ClosureFilters, Filters};
use crate::utils::validation::ValidateRequest;
use chrono::DateTime;
use event_protos::event::v1::{
    CreateClosureRequest, CreateClosureResponse, DeleteClosureRequest, DeleteClosureResponse,
    ListClosuresRequest, ListClosuresResponse, UpdateClosureRequest, UpdateClosureResponse,
};
use tonic::metadata::MetadataMap;
use uuid::Uuid;

pub async fn create_closure(
    request: CreateClosureRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<CreateClosureResponse, ApiError> {
    request.validate()?;

    let closing_from = DateTime::parse_from_rfc3339(&request.closing_from)?.naive_utc();
    let closing_to = DateTime::parse_from_rfc3339(&request.closing_to)?.naive_utc();

    let closure = conn
        .create_closure(&ClosureInsert {
            closing_from,
            closing_to,
            organizer_key: request.organizer_key,
        })
        .await?;

    Ok(CreateClosureResponse {
        closure: Some(closure.to_response(get_meta_timezone(meta))),
    })
}

pub async fn list_closures(
    request: ListClosuresRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<ListClosuresResponse, ApiError> {
    request.validate()?;

    let filters: Filters<ClosureFilters> = request.filters.into();

    let closures = conn.get_closures_with_filters(&filters).await?;

    Ok(ListClosuresResponse {
        closures: closures
            .into_iter()
            .map(|closure| closure.to_response(get_meta_timezone(meta)))
            .collect(),
    })
}

pub async fn update_closure(
    request: UpdateClosureRequest,
    meta: &MetadataMap,
    conn: &mut PgPooledConnection,
) -> Result<UpdateClosureResponse, ApiError> {
    request.validate()?;

    let closure_id = Uuid::parse_str(&request.id)?;

    let closing_from = match request.closing_from.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.closing_from)?.naive_utc()),
    };
    let closing_to = match request.closing_to.is_empty() {
        true => None,
        false => Some(DateTime::parse_from_rfc3339(&request.closing_to)?.naive_utc()),
    };

    let updated_closure = conn
        .update_closure(
            closure_id,
            &ClosureUpdate {
                closing_from,
                closing_to,
            },
        )
        .await?;

    Ok(UpdateClosureResponse {
        closure: Some(updated_closure.to_response(get_meta_timezone(meta))),
    })
}

pub async fn delete_closure(
    request: DeleteClosureRequest,
    conn: &mut PgPooledConnection,
) -> Result<DeleteClosureResponse, ApiError> {
    request.validate()?;

    let closure_id = Uuid::parse_str(&request.id)?;

    conn.delete_closure(closure_id).await?;

    Ok(DeleteClosureResponse {
        message: "Closure deleted successfully".to_string(),
    })
}
