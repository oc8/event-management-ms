use tonic::Status;
use protos::booking::v1::{CreateEventRequest, CreateEventResponse};
use crate::database::PgPooledConnection;
use crate::validations::validate_create_event_request;

pub fn create_event(
    request: CreateEventRequest,
    _conn: &mut PgPooledConnection
) -> Result<CreateEventResponse, Status> {
    validate_create_event_request(&request)?;

    Ok(CreateEventResponse{
        event: None
    })
}
