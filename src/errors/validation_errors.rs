use tonic::{Code};

pub struct ValidationError {
    pub message: &'static str,
    pub fields: Vec<&'static str>,
    pub code: ValidationErrorCode,
}

#[derive(Debug, PartialEq)]
pub enum ValidationErrorCode {
    InvalidRfc3339,
    InvalidRange,
    InvalidId,
    Required,
    InvalidTimezone,
    InvalidRecurrenceRule,
    InvalidEventType,
    InvalidDate,
    InvalidLength,
}

pub fn validation_error(fields: Vec<&'static str>, message: &'static str, code: ValidationErrorCode) -> ValidationError {
    ValidationError {
        fields,
        message,
        code
    }
}

pub fn format_validation_errors(errors: Vec<ValidationError>) -> tonic::Status {
    let mut errors_json = String::from("{ \"errors\": [");
    for error in errors {
        errors_json.push_str(&format!("{{ \"message\": \"{}\", \"fields\": \"{:?}\", \"code\": \"{:?}\" }},", error.message, error.fields, error.code));
    }

    errors_json.pop(); // remove last comma
    errors_json.push_str("]}");

    tonic::Status::new(Code::InvalidArgument, errors_json)
}
