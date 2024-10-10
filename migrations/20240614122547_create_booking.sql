CREATE TABLE booking (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_holder_key VARCHAR(255) NOT NULL,
    organizer_key VARCHAR(255) NOT NULL,
    slot_id uuid NOT NULL,
    event_id uuid NOT NULL,
    date_time TIMESTAMP NOT NULL,
    persons INT DEFAULT 1 NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
    FOREIGN KEY (slot_id) REFERENCES event_slot(id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES event(id) ON DELETE CASCADE
);

SELECT manage_updated_at('booking');