use protos::booking::v1::{Filters};
use crate::errors;
use crate::errors::{ApiError};

pub fn validate_date_filters(filters: &Option<Filters>) -> Result<(), Vec<ApiError>> {
    if filters.is_none() {
        return Ok(());
    }

    let filters = filters.as_ref().unwrap();

    let mut errors = Vec::new();

    let from = chrono::NaiveDate::parse_from_str(&filters.from, "%Y-%m-%d");
    if !filters.from.is_empty() {
        match from {
            Ok(_) => (),
            Err(_) => errors.push(errors::INVALID_DATE),
        }
    }

    let to = chrono::NaiveDate::parse_from_str(&filters.to, "%Y-%m-%d");
    if !filters.to.is_empty() {
        match to {
            Ok(_) => (),
            Err(_) => errors.push(errors::INVALID_DATE),
        }
    }

    if !from.is_err() && !to.is_err() {
        if from.unwrap() > to.unwrap() {
            errors.push(errors::INVALID_DATE_RANGE);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}