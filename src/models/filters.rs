use chrono::NaiveDateTime;
use protos::booking::v1::{EventStatus, EventType, Filters as FiltersProto};

#[derive(Debug, Clone)]
pub struct Filters<T> {
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub type_filters: T,
}

impl<T> Filters<T> {
    pub fn new(
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
        type_filters: T,
    ) -> Self {
        Filters {
            from,
            to,
            type_filters,
        }
    }
}

trait AdditionalFilterFields {
    type TypeFilters;

    fn create_type_filters(
        organizer_key: Option<String>,
        status: Option<EventStatus>,
        event_type: Option<EventType>,
        only_active: Option<bool>,
        booking_holder_key: Option<String>,
        slot_id: Option<String>,
    ) -> Self::TypeFilters;
}

#[derive(Default, Debug, Clone)]
pub struct EventFilters {
    pub organizer_key: Option<String>,
    pub status: Option<EventStatus>,
    pub event_type: Option<EventType>,
    pub only_active: Option<bool>,
}

impl AdditionalFilterFields for EventFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        organizer_key: Option<String>,
        status: Option<EventStatus>,
        event_type: Option<EventType>,
        only_active: Option<bool>,
        _: Option<String>,
        _: Option<String>,
    ) -> Self {
        EventFilters {
            organizer_key,
            status,
            event_type,
            only_active,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BookingFilters {
    pub organizer_key: Option<String>,
    pub booking_holder_key: Option<String>,
    pub slot_id: Option<String>,
}

impl AdditionalFilterFields for BookingFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        organizer_key: Option<String>,
        _: Option<EventStatus>, // Placeholder for EventFilters fields
        _: Option<EventType>,
        _: Option<bool>,
        booking_holder_key: Option<String>,
        slot_id: Option<String>,
    ) -> Self {
        BookingFilters {
            organizer_key,
            booking_holder_key,
            slot_id,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClosureFilters {
    pub organizer_key: Option<String>,
}

impl AdditionalFilterFields for ClosureFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        organizer_key: Option<String>,
        _: Option<EventStatus>, // Placeholder for EventFilters fields
        _: Option<EventType>,
        _: Option<bool>,
        _: Option<String>, // Placeholder for BookingFilters fields
        _: Option<String>,
    ) -> Self {
        ClosureFilters {
            organizer_key,
        }
    }
}


impl<T> From<Option<FiltersProto>> for Filters<T>
    where
        T: AdditionalFilterFields<TypeFilters = T>,
        T::TypeFilters: Default,
{
    fn from(proto: Option<FiltersProto>) -> Self {
        let proto = proto.unwrap();

        let from = if proto.from.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(format!("{}T00:00:00", proto.from).as_str(), "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let to = if proto.to.is_empty() {
            None
        } else {
            Some(NaiveDateTime::parse_from_str(format!("{}T23:59:59", proto.to).as_str(), "%Y-%m-%dT%H:%M:%S").unwrap())
        };

        let type_filters = T::create_type_filters(
            Some(proto.organizer_key),
            Some(EventStatus::try_from(proto.status).unwrap()),
            Some(EventType::try_from(proto.event_type).unwrap()),
            Some(proto.only_active),
            Some(proto.booking_holder_key),
            Some(proto.slot_id),
        );

        Filters {
            from,
            to,
            type_filters,
        }
    }
}
