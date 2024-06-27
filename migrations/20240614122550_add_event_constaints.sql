ALTER TABLE event
    ADD CONSTRAINT end_time_after_start_time CHECK (end_time > start_time),
    ADD CONSTRAINT slot_capacity_positive CHECK (slot_capacity > 0),
    ADD CONSTRAINT capacity_positive CHECK (capacity > 0);
