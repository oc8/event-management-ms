ALTER TABLE bookings
    ADD CONSTRAINT persons_positive CHECK (persons > 0);