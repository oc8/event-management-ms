CREATE TABLE event_slot (
     id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
     event_id uuid NOT NULL,
     start_time TIME NOT NULL,
     end_time TIME NOT NULL,
     capacity INT DEFAULT 1 NOT NULL,
     created_at TIMESTAMP DEFAULT NOW() NOT NULL,
     updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
     FOREIGN KEY (event_id) REFERENCES event(id) ON DELETE CASCADE
);

SELECT manage_updated_at('event_slot');