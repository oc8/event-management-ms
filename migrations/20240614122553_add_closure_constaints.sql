ALTER TABLE closures
    ADD CONSTRAINT to_time_after_from_time CHECK (closing_to > closing_from);