CREATE TYPE event_type AS ENUM (
    'EVENT_TYPE_LOCATION_UNSPECIFIED',
    'EVENT_TYPE_EVENT',
    'EVENT_TYPE_TASK',
    'EVENT_TYPE_MEETING'
);

CREATE TYPE event_status AS ENUM (
    'EVENT_STATUS_UNSPECIFIED',
    'EVENT_STATUS_ACTIVE',
    'EVENT_STATUS_CANCELED',
    'EVENT_STATUS_FULL'
);

CREATE TABLE events (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    status VARCHAR(255) DEFAULT 'EVENT_STATUS_ACTIVE' NOT NULL,
    event_type VARCHAR(255) DEFAULT 'EVENT_TYPE_EVENT' NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    recurrence_rule VARCHAR(255) DEFAULT NULL,
    timezone VARCHAR(255) NOT NULL,
    organizer_key VARCHAR(255) NOT NULL,
    max_guests INT DEFAULT NULL,
    canceled_by VARCHAR(255) DEFAULT NULL,
    canceled_at TIMESTAMP DEFAULT NULL,
    canceled_reason VARCHAR(255) DEFAULT NULL,
    slot_duration INTERVAL DEFAULT NULL,
    max_persons_per_slot INT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);