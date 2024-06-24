use chrono::NaiveDateTime;
use uuid::Uuid;
use protos::event::v1::{EventStatus, EventType, Filters as FiltersProto};
use crate::errors::{ValidationErrorKind, ValidationErrorMessage};

#[derive(Debug, Clone)]
pub struct Filters<T> {
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
    pub organizer_key: Option<String>,
    pub type_filters: T,
}

impl<T> Filters<T> {
    pub fn new(
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
        organizer_key: Option<String>,
        type_filters: T,
    ) -> Self {
        Filters {
            from,
            to,
            organizer_key,
            type_filters,
        }
    }
}

trait AdditionalFilterFields {
    type TypeFilters;

    fn create_type_filters(
        status: Option<EventStatus>,
        event_type: Option<EventType>,
        booking_holder_key: Option<String>,
        slot_id: Option<Uuid>,
    ) -> Self::TypeFilters;
}

#[derive(Default, Debug, Clone)]
pub struct EventFilters {
    pub status: Option<EventStatus>,
    pub event_type: Option<EventType>,
}

impl AdditionalFilterFields for EventFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        status: Option<EventStatus>,
        event_type: Option<EventType>,
        _: Option<String>,
        _: Option<Uuid>,
    ) -> Self {
        EventFilters {
            status,
            event_type,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BookingFilters {
    pub booking_holder_key: Option<String>,
    pub slot_id: Option<Uuid>,
}

impl AdditionalFilterFields for BookingFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        _: Option<EventStatus>, // Placeholder for EventFilters fields
        _: Option<EventType>,
        booking_holder_key: Option<String>,
        slot_id: Option<Uuid>,
    ) -> Self {
        BookingFilters {
            booking_holder_key,
            slot_id,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClosureFilters { }

impl AdditionalFilterFields for ClosureFilters {
    type TypeFilters = Self;

    fn create_type_filters(
        _: Option<EventStatus>, // Placeholder for EventFilters fields
        _: Option<EventType>,
        _: Option<String>, // Placeholder for BookingFilters fields
        _: Option<Uuid>,
    ) -> Self {
        ClosureFilters { }
    }
}

impl<T> From<Option<FiltersProto>> for Filters<T>
    where
        T: AdditionalFilterFields<TypeFilters = T>,
        T::TypeFilters: Default,
{
    fn from(proto: Option<FiltersProto>) -> Self {
        let proto = proto.unwrap();

        let from = match proto.from.is_empty() {
            true => chrono::Utc::now().naive_utc(),
            false => NaiveDateTime::parse_from_str(format!("{}T00:00:00", proto.from).as_str(), "%Y-%m-%dT%H:%M:%S").unwrap()
        };

        let to = match proto.to.is_empty() {
            true => from + chrono::Duration::days(7),
            false => NaiveDateTime::parse_from_str(format!("{}T23:59:59", proto.to).as_str(), "%Y-%m-%dT%H:%M:%S").unwrap()
        };

        let organizer_key = match proto.organizer_key.is_empty() {
            true => None,
            false => Some(proto.organizer_key)
        };

        let slot_id = match proto.slot_id.is_empty() {
            true => None,
            false => Some(Uuid::parse_str(proto.slot_id.as_str()).unwrap())
        };

        let type_filters = T::create_type_filters(
            Some(EventStatus::try_from(proto.status).unwrap()),
            Some(EventType::try_from(proto.event_type).unwrap()),
            Some(proto.booking_holder_key),
            slot_id,
        );

        Filters {
            from: Some(from),
            to: Some(to),
            organizer_key,
            type_filters,
        }
    }
}

pub fn validate_date_filters(filters: &Option<FiltersProto>) -> Result<(), Vec<ValidationErrorKind>> {
    if filters.is_none() {
        return Ok(());
    }

    let filters = filters.as_ref().unwrap();

    let mut errors = Vec::new();

    let from = chrono::NaiveDate::parse_from_str(&filters.from, "%Y-%m-%d");
    if !filters.from.is_empty() {
        match from {
            Ok(_) => (),
            Err(_) => errors.push(ValidationErrorKind::InvalidFormat("filters.from".to_string(), ValidationErrorMessage::InvalidDateTime()))
        }
    }

    let to = chrono::NaiveDate::parse_from_str(&filters.to, "%Y-%m-%d");
    if !filters.to.is_empty() {
        match to {
            Ok(_) => (),
            Err(_) => errors.push(ValidationErrorKind::InvalidFormat("filters.to".to_string(), ValidationErrorMessage::InvalidDateTime()))
        }
    }

    if !from.is_err() && !to.is_err() {
        if from.unwrap() > to.unwrap() {
            errors.push(ValidationErrorKind::InvalidDateRange("from, to".to_string(), "from must be before to".to_string()))
        }
    }

    if !filters.slot_id.is_empty() {
        match Uuid::parse_str(filters.slot_id.as_str()) {
            Ok(_) => (),
            Err(_) => errors.push(ValidationErrorKind::InvalidFormat("filters.slot_id".to_string(), ValidationErrorMessage::InvalidUuid()))
        }
    }

    if !errors.is_empty() {
        return Err(errors)
    }

    Ok(())
}