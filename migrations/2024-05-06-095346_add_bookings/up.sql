CREATE TABLE bookings (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_holder_key VARCHAR(255) NOT NULL,
    slot_id uuid NOT NULL,
    date_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    FOREIGN KEY (slot_id) REFERENCES event_slots(id)
);