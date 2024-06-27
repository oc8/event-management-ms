CREATE TABLE closure (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    closing_from TIMESTAMP NOT NULL,
    closing_to TIMESTAMP NOT NULL,
    organizer_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

SELECT manage_updated_at('closure');