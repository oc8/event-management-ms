CREATE TABLE bookings (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    booking_holder_key VARCHAR(255) NOT NULL,
    slot_id uuid NOT NULL,
    number_of_people INT,
    message TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    FOREIGN KEY (slot_id) REFERENCES event_slots(id)
);