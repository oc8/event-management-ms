CREATE TABLE closing_exceptions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id uuid NOT NULL,
    closing_from TIMESTAMP NOT NULL,
    closing_to TIMESTAMP NOT NULL,
    reason text NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY (event_id) REFERENCES events(id)
);