ALTER TABLE event_slots
    ADD CONSTRAINT end_time_after_start_time CHECK (end_time > start_time),
    ADD CONSTRAINT capacity_positive CHECK (capacity > 0);