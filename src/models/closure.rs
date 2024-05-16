use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;
use protos::booking::v1::{TimeData};
use crate::schema::{closures};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = closures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Closure {
    pub id: Uuid,
    pub closing_from: NaiveDateTime,
    pub closing_to: NaiveDateTime,
    pub organizer_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = closures)]
pub struct NewClosure<'a> {
    pub closing_from: &'a NaiveDateTime,
    pub closing_to: &'a NaiveDateTime,
    pub organizer_key: &'a str,
}

impl Closure {
    pub fn create(
        conn: &mut PgConnection,
        exception: NewClosure,
    ) -> Result<Closure, diesel::result::Error> {
        match diesel::insert_into(closures::table)
            .values(exception)
            .returning(Closure::as_returning())
            .get_result(conn)
        {
            Ok(exception) => Ok(exception),
            Err(e) => {
                log::error!("Failed to create closure: {}", e);
                Err(e)
            },
        }
    }

    pub fn find_by_id(conn: &mut PgConnection, slot_id: Uuid) -> Option<Closure> {
        closures::dsl::closures
            .select(Closure::as_select())
            .filter(closures::dsl::id.eq(slot_id))
            .first(conn)
            .ok()
    }

    pub fn find_by_organizer_key(conn: &mut PgConnection, organizer_key: &str) -> Vec<Closure> {
        closures::dsl::closures
            .select(Closure::as_select())
            .filter(closures::dsl::organizer_key.eq(organizer_key))
            .load(conn)
            .expect("Error loading closures")
    }
}

impl From<Closure> for protos::booking::v1::Closure {
    fn from(closure: Closure) -> Self {
        let mut proto_closure = protos::booking::v1::Closure::default();

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
