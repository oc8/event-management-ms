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

pub const INVALID_EVENT_START_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_event_start_date",
    message: "Invalid event start date"
};

pub const INVALID_EVENT_END_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_event_end_date",
    message: "Invalid event end date"
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

pub const INVALID_MAX_GUESTS: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_max_guests",
    message: "Invalid max guests"
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

pub const INVALID_CLOSING_START_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_closing_start_date",
    message: "Invalid closing start date"
};

pub const INVALID_CLOSING_END_DATE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_closing_end_date",
    message: "Invalid closing end date"
};

pub const INVALID_CLOSING_DATE_RANGE: ApiError = ApiError {
    grpc_code: Code::InvalidArgument,
    code: "invalid_closing_date_range",
    message: "Invalid closing date range"
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