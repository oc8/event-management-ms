use chrono::NaiveDateTime;
use protos::booking::v1::{EventStatus, EventType, Filters as FiltersProto};

#[derive(Debug)]
pub struct Filters {
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub organizer_key: Option<String>,
    pub status: Option<EventStatus>,
    pub event_type: Option<EventType>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Filters {
    pub fn new(
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
        organizer_key: Option<String>,
        status: Option<EventStatus>,
        event_type: Option<EventType>,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Self {
        Filters {
            from,
            to,
            organizer_key,
            status,
            event_type,
            limit,
            offset,
        }
    }
}

impl From<Option<FiltersProto>> for Filters {
    fn from(proto: Option<FiltersProto>) -> Self {
        let proto = proto.unwrap();

        let from = if proto.from.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(&proto.from, "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let to = if proto.to.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(&proto.to, "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let limit = if proto.limit == 0 {
            Some(50)
        } else {
            Some(proto.limit)
        };

        Filters {
            from,
            to,
            organizer_key: Some(proto.organizer_key),
            status: Some(EventStatus::try_from(proto.status).unwrap()),
            event_type: Some(EventType::try_from(proto.event_type).unwrap()),
            limit,
            offset: Some(proto.offset),
        }
    }
}
