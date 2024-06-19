CREATE TYPE event_type AS ENUM (
    'unspecified',
    'event',
    'task',
    'meeting'
);

CREATE TYPE event_status AS ENUM (
    'unspecified',
    'active',
    'canceled',
    'closed',
    'full',
    'disabled'
);

CREATE TABLE events (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    status event_status DEFAULT 'active' NOT NULL,
    event_type event_type DEFAULT 'event' NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    recurrence_rule VARCHAR(255) DEFAULT NULL,
    timezone VARCHAR(255) NOT NULL,
    organizer_key VARCHAR(255) NOT NULL,
    canceled_by VARCHAR(255) DEFAULT NULL,
    canceled_at TIMESTAMP DEFAULT NULL,
    canceled_reason VARCHAR(255) DEFAULT NULL,
    slot_duration INTERVAL DEFAULT NULL,
    slot_capacity INT DEFAULT NULL,
    capacity INT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

SELECT manage_updated_at('events');