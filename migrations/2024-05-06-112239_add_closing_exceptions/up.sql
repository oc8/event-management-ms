CREATE TABLE closing_exceptions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    closing_from TIMESTAMP NOT NULL,
    closing_to TIMESTAMP NOT NULL,
    reason text,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);