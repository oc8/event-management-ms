use async_trait::async_trait;
use sqlx::{PgConnection, Postgres, QueryBuilder};
use uuid::Uuid;
use crate::errors::ApiError;
use crate::server::services::v1::closure::closure_model::{Closure, ClosureInsert, ClosureRepository, ClosureUpdate};
use crate::utils::filters::{ClosureFilters, Filters};

#[async_trait]
impl ClosureRepository for PgConnection {
    async fn create_closure(&mut self, closure: &ClosureInsert) -> Result<Closure, ApiError> {
        let closure = sqlx::query_as!(
            Closure,
            r#"
            INSERT INTO closure (closing_from, closing_to, organizer_key)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            closure.closing_from,
            closure.closing_to,
            closure.organizer_key
        )
        .fetch_one(self)
        .await?;

        log::debug!("Created closure: {:?}", closure);

        Ok(closure)
    }

    async fn get_closure_by_id(&mut self, closure_id: Uuid) -> Result<Closure, ApiError> {
        let closure = sqlx::query_as!(
            Closure,
            r#"
            SELECT * FROM closure
            WHERE id = $1
            "#,
            closure_id
        )
        .fetch_one(self)
        .await?;

        log::debug!("Found closure: {:?}", closure);

        Ok(closure)
    }

    async fn get_closures_with_filters(&mut self, filters: &Filters<ClosureFilters>) -> Result<Vec<Closure>, ApiError> {
        log::debug!("Finding closures with filters: {:?}", filters);

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT * FROM closure
            WHERE 1 = 1
            "#,
        );

        if let Some(ref organizer_key) = filters.organizer_key {
            query_builder.push(" AND organizer_key = ");
            query_builder.push_bind(organizer_key);
        }
        if let Some(ref closing_from) = filters.from {
            query_builder.push(" AND closing_from >= ");
            query_builder.push_bind(closing_from);
        }
        // if let Some(ref closing_to) = filters.to {
        //     query_builder.push(" AND closing_to <= ");
        //     query_builder.push_bind(closing_to);
        // }

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        let closures = query_builder.build_query_as::<Closure>()
            .fetch_all(self)
            .await?;

        Ok(closures)
    }

    async fn update_closure(&mut self, closure_id: Uuid, closure: &ClosureUpdate) -> Result<Closure, ApiError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            UPDATE closure
            "#,
        );

        // TODO: This is not the best way to build the query but it works for now
        let mut is_first = true;

        if let Some(closing_from) = &closure.closing_from {
            if is_first {
                query_builder.push(" SET closing_from = ");
                is_first = false;
            } else {
                query_builder.push(", closing_from = ");
            }
            query_builder.push_bind(closing_from);
        }
        if let Some(closing_to) = &closure.closing_to {
            if is_first {
                query_builder.push(" SET closing_to = ");
            } else {
                query_builder.push(", closing_to = ");
            }
            query_builder.push_bind(closing_to);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(closure_id);

        query_builder.push(" RETURNING *");

        let closure = query_builder.build_query_as::<Closure>()
            .fetch_one(self).await?;

        log::debug!("Generated SQL Query: {}", query_builder.sql());

        Ok(closure)
    }

    async fn delete_closure(&mut self, id: Uuid) -> Result<u64, ApiError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM closure
            WHERE id = $1
            "#,
            id
        )
        .execute(self)
        .await?;

        log::debug!("Deleted closure with id: {}", id);

        Ok(result.rows_affected())
    }
}