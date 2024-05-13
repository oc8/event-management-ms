CREATE TABLE closures (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    closing_from TIMESTAMP NOT NULL,
    closing_to TIMESTAMP NOT NULL,
    reason text,
    organizer_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);