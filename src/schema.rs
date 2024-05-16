// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Uuid,
        #[max_length = 255]
        booking_holder_key -> Varchar,
        slot_id -> Uuid,
        date_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    closures (id) {
        id -> Uuid,
        closing_from -> Timestamp,
        closing_to -> Timestamp,
        reason -> Nullable<Text>,
        #[max_length = 255]
        organizer_key -> Varchar,
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
        max_persons -> Int4,
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
        #[max_length = 255]
        canceled_by -> Nullable<Varchar>,
        canceled_at -> Nullable<Timestamp>,
        #[max_length = 255]
        canceled_reason -> Nullable<Varchar>,
        slot_duration -> Nullable<Interval>,
        max_persons_per_slot -> Nullable<Int4>,
        max_persons -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bookings -> event_slots (slot_id));
diesel::joinable!(event_slots -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    closures,
    event_slots,
    events,
);
