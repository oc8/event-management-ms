use uuid::Uuid;
use crate::errors::{ApiError};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use protos::event::v1::{TimeData};
use crate::utils::filters::{ClosureFilters, Filters};

/// Defines the database structure of a closure.
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct Closure {
    pub id: Uuid,
    pub closing_from: NaiveDateTime,
    pub closing_to: NaiveDateTime,
    pub organizer_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Closure> for protos::event::v1::Closure {
    fn from(closure: Closure) -> Self {
        let mut proto_closure = protos::event::v1::Closure::default();

        proto_closure.id = closure.id.to_string();
        proto_closure.closing_from = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(closure.closing_from, Utc).to_rfc3339()
        });

        proto_closure.closing_to = Some(TimeData {
            timezone: "UTC".to_string(), // TODO: Get timezone from event
            date_time: DateTime::<Utc>::from_naive_utc_and_offset(closure.closing_to, Utc).to_rfc3339()
        });
        proto_closure.organizer_key = closure.organizer_key;
        proto_closure.created_at = closure.created_at.and_utc().timestamp();
        proto_closure.updated_at = closure.updated_at.and_utc().timestamp();

        proto_closure
    }
}

/// Defines a closure structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub(crate) struct ClosureInsert {
    pub closing_from: NaiveDateTime,
    pub closing_to: NaiveDateTime,
    pub organizer_key: String,
}

/// Defines a closure structure that can be updated in the database.
#[derive(Debug, PartialEq)]
pub(crate) struct ClosureUpdate {
    pub closing_from: Option<NaiveDateTime>,
    pub closing_to: Option<NaiveDateTime>,
}

#[async_trait]
pub(crate) trait ClosureRepository: Send + Sync + 'static {
    /// Attempts to create a new closure in the database.
    ///
    /// # Parameters
    /// A struct containing the closure data to be inserted.
    ///
    /// ## Success
    /// A struct containing the newly created closure.
    ///
    /// ## Errors
    /// An error could occur if the closure already exists, or a failure occurred with the
    /// database.
    async fn create_closure(
        &mut self,
        closure: &ClosureInsert,
    ) -> Result<Closure, ApiError>;

    /// Attempts to retrieve a closure by its id.
    ///
    /// # Parameters
    /// The id of the closure to be retrieved.
    ///
    /// ## Success
    /// A struct containing the closure.
    ///
    /// ## Errors
    /// An error could occur if the closure does not exist, or a failure occurred with the
    /// database.
    async fn get_closure_by_id(
        &mut self,
        closure_id: Uuid,
    ) -> Result<Closure, ApiError>;

    /// Attempts to retrieve a list of closures with filters.
    ///
    /// # Parameters
    /// A struct containing the filters to be applied.
    ///
    /// ## Success
    /// A vector containing the closures that match the filter.
    ///
    /// ## Errors
    /// An error could occur if a failure occurred with the database.
    async fn get_closures_with_filters(
        &mut self,
        filters: &Filters<ClosureFilters>
    ) -> Result<Vec<Closure>, ApiError>;

    /// Attempts to update a closure by its id.
    ///
    /// # Parameters
    /// The id of the closure to be updated.
    /// A struct containing the details of the closure to be updated.
    ///
    /// ## Success
    /// A struct containing the updated closure.
    ///
    /// ## Errors
    /// An error could occur if the event does not exist, or a failure occurred with the
    /// database.
    async fn update_closure(
        &mut self,
        closure_id: Uuid,
        closure: &ClosureUpdate,
    ) -> Result<Closure, ApiError>;

    /// Attempts to delete a closure by its id.
    ///
    /// # Parameters
    /// The id of the closure to be deleted.
    ///
    /// ## Success
    /// The number of rows affected by the delete operation.
    ///
    /// ## Errors
    /// An error could occur if the closure does not exist, or a failure occurred with the
    /// database.
    async fn delete_closure(
        &mut self,
        id: Uuid,
    ) -> Result<u64, ApiError>;
}
