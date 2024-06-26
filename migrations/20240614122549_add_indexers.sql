CREATE INDEX idx_events_start_time ON events(start_time);
CREATE INDEX idx_events_end_time ON events(end_time);
CREATE INDEX idx_events_organizer_key ON events(organizer_key);
CREATE INDEX idx_events_status ON events(status);
CREATE INDEX idx_events_event_type ON events(event_type);

CREATE INDEX idx_event_slots_event_id ON event_slots(event_id);
CREATE INDEX idx_event_slots_start_time ON event_slots(start_time);
CREATE INDEX idx_event_slots_end_time ON event_slots(end_time);

CREATE INDEX idx_bookings_slot_id ON bookings(slot_id);
CREATE INDEX idx_bookings_booking_holder_key ON bookings(booking_holder_key);
CREATE INDEX idx_bookings_organizer_key ON bookings(organizer_key);

CREATE INDEX idx_closures_closing_from ON closures(closing_from);
CREATE INDEX idx_closures_closing_to ON closures(closing_to);
CREATE INDEX idx_closures_organizer_key ON closures(organizer_key);
