use protos::booking::v1::{Filters};
use crate::errors;
use crate::errors::{validation_error, ValidationError};

pub fn validate_date_filters(filters: &Option<Filters>) -> Result<(), Vec<ValidationError>> {
    if filters.is_none() {
        return Ok(());
    }

    let filters = filters.as_ref().unwrap();

    let mut errors = Vec::new();

    let from = chrono::NaiveDate::parse_from_str(&filters.from, "%Y-%m-%d");
    if !filters.from.is_empty() {
        match from {
            Ok(_) => (),
            Err(_) => errors.push(validation_error(vec!["from"], "invalid from date, must be in the format YYYY-MM-DD", errors::ValidationErrorCode::InvalidDate)),
        }
    }

    let to = chrono::NaiveDate::parse_from_str(&filters.to, "%Y-%m-%d");
    if !filters.to.is_empty() {
        match to {
            Ok(_) => (),
            Err(_) => errors.push(validation_error(vec!["to"], "invalid to date, must be in the format YYYY-MM-DD", errors::ValidationErrorCode::InvalidDate)),
        }
    }

    if !from.is_err() && !to.is_err() {
        if from.unwrap() > to.unwrap() {
            errors.push(validation_error(vec!["from", "to"], "from date must be before to date", errors::ValidationErrorCode::InvalidRange));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}