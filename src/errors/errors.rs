use std::string::ToString;
use tonic::{Code};

pub struct ApiError {
    pub grpc_code: Code,
    pub code: &'static str,
    pub message: &'static str
}

pub const INTERNAL: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "internal",
    message: "Internal Server Error"
};

pub const REDIS_CONNECTION_FAILURE: ApiError = ApiError {
    grpc_code: Code::DataLoss,
    code: "redis_connection_failure",
    message: "Failed to connect to Redis"
};

pub const DATABASE_CONNECTION_FAILURE: ApiError = ApiError {
    grpc_code: Code::DataLoss,
    code: "database_connection_failure",
    message: "Failed to connect to Database"
};

pub const INVALID_EVENT_NAME: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_event_name",
    message: "Invalid event name"
};

pub const INVALID_TIMEZONE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_timezone",
    message: "Invalid timezone"
};

pub const INVALID_RECURRENCE_RULE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_recurrence_rule",
    message: "Invalid recurrence rule"
};

pub const INVALID_CAPACITY: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_capacity",
    message: "Invalid capacity"
};

pub const INVALID_EVENT_TYPE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_event_type",
    message: "Invalid event type"
};

pub const INVALID_EVENT_ID: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_event_id",
    message: "Invalid event id"
};

pub const EVENT_NOT_FOUND: ApiError = ApiError {
    grpc_code: Code::NotFound,
    code: "event_not_found",
    message: "Event not found"
};

pub const SLOT_NOT_FOUND: ApiError = ApiError {
    grpc_code: Code::NotFound,
    code: "slot_not_found",
    message: "Slot not found"
};

pub const BOOKING_NOT_FOUND: ApiError = ApiError {
    grpc_code: Code::NotFound,
    code: "booking_not_found",
    message: "Booking not found"
};

pub const INVALID_BOOKING_HOLDER_KEY: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_booking_holder_key",
    message: "Invalid booking holder key"
};

pub const INVALID_SLOT_ID: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_slot_id",
    message: "Invalid slot id"
};

pub const INVALID_BOOKING_ID: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_booking_id",
    message: "Invalid booking id"
};

pub const INVALID_ORGANIZER_KEY: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_organizer_key",
    message: "Invalid organizer key"
};

pub const INVALID_FILTERS: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_filters",
    message: "Invalid filters"
};

pub const INVALID_BOOKING_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_booking_date",
    message: "Invalid booking date"
};

pub const INVALID_SLOT_DURATION: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_slot_duration",
    message: "Invalid slot duration"
};

pub const BOOKING_ALREADY_EXISTS: ApiError = ApiError {
    grpc_code: Code::AlreadyExists,
    code: "booking_already_exists",
    message: "Booking already exists"
};

pub const BOOKING_DATE_IN_PAST: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "booking_date_in_past",
    message: "Booking date is in the past"
};

pub const BOOKING_DATE_TIME_MISMATCH: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "booking_date_time_mismatch",
    message: "Booking date and time mismatch"
};

pub const BOOKING_CAPACITY_FULL: ApiError = ApiError {
    grpc_code: Code::ResourceExhausted,
    code: "booking_capacity_full",
    message: "Booking capacity is full"
};

pub const INVALID_PERSONS_NUMBER: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_persons_number",
    message: "Invalid persons number"
};

pub const EVENT_CREATION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "event_creation_failed",
    message: "Failed to create event"
};

pub const INVALID_DATETIME: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_date",
    message: "Invalid datetime format: Y-m-dTH:M:S"
};

pub const INVALID_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_date",
    message: "Invalid date format: Y-m-d"
};

pub const INVALID_DATE_RANGE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_date_range",
    message: "Invalid date range"
};

pub const EVENT_UPDATE_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "event_update_failed",
    message: "Failed to update event"
};

pub const EVENT_DELETION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "event_deletion_failed",
    message: "Failed to delete event"
};

pub const BOOKING_CREATION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "booking_creation_failed",
    message: "Failed to create booking"
};

pub const BOOKING_DELETION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "booking_deletion_failed",
    message: "Failed to delete booking"
};

pub const CLOSURE_CREATION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "closure_creation_failed",
    message: "Failed to create closure"
};

pub const CLOSURE_NOT_FOUND: ApiError = ApiError {
    grpc_code: Code::NotFound,
    code: "closure_not_found",
    message: "Closure not found"
};

pub const INVALID_CLOSURE_ID: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_closure_id",
    message: "Invalid closure id"
};

pub const CLOSURE_UPDATE_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "closure_update_failed",
    message: "Failed to update closure"
};

pub const CLOSURE_DELETION_FAILED: ApiError = ApiError {
    grpc_code: Code::Internal,
    code: "closure_deletion_failed",
    message: "Failed to delete closure"
};

// format error to json like { "code": "invalid_slot_duration", "message": "Invalid slot duration" }
pub fn format_error(error: ApiError) -> tonic::Status {
    let error_json = format!("{{ \"code\": \"{}\", \"message\": \"{}\" }}", error.code, error.message);
    tonic::Status::new(error.grpc_code, error_json)
}

// format errors to json like { "errors": [{ "code": "invalid_slot_duration", "message": "Invalid slot duration" }] }
pub fn format_errors(code: Code, errors: Vec<ApiError>) -> tonic::Status {
    let mut errors_json = String::from("{ \"errors\": [");
    for error in errors {
        errors_json.push_str(&format!("{{ \"code\": \"{}\", \"message\": \"{}\" }},", error.code, error.message));
    }

    errors_json.pop(); // remove last comma
    errors_json.push_str("]}");

    tonic::Status::new(code, errors_json)
}