ALTER TABLE booking
    ADD CONSTRAINT persons_positive CHECK (persons > 0);