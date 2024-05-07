// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        slot_id -> Uuid,
        number_of_people -> Int4,
        message -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    closing_exceptions (id) {
        id -> Uuid,
        event_id -> Uuid,
        closing_from -> Timestamp,
        closing_to -> Timestamp,
        reason -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    event_slots (id) {
        id -> Uuid,
        event_id -> Uuid,
        start_time -> Time,
        end_time -> Time,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        #[max_length = 255]
        event_type -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        #[max_length = 255]
        recurrence_rule -> Nullable<Varchar>,
        #[max_length = 255]
        timezone -> Varchar,
        #[max_length = 255]
        organizer_key -> Varchar,
        max_guests -> Nullable<Int4>,
        #[max_length = 255]
        canceled_by -> Nullable<Varchar>,
        canceled_at -> Nullable<Timestamp>,
        #[max_length = 255]
        canceled_reason -> Nullable<Varchar>,
        slot_duration -> Nullable<Interval>,
        max_persons_per_slot -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bookings -> event_slots (slot_id));
diesel::joinable!(closing_exceptions -> events (event_id));
diesel::joinable!(event_slots -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    closing_exceptions,
    event_slots,
    events,
);
