CREATE INDEX idx_event_start_time ON event(start_time);
CREATE INDEX idx_event_end_time ON event(end_time);
CREATE INDEX idx_event_organizer_key ON event(organizer_key);
CREATE INDEX idx_event_status ON event(status);
CREATE INDEX idx_event_event_type ON event(event_type);

CREATE INDEX idx_event_slot_event_id ON event_slot(event_id);
CREATE INDEX idx_event_slot_start_time ON event_slot(start_time);
CREATE INDEX idx_event_slot_end_time ON event_slot(end_time);

CREATE INDEX idx_booking_slot_id ON booking(slot_id);
CREATE INDEX idx_booking_booking_holder_key ON booking(booking_holder_key);
CREATE INDEX idx_booking_organizer_key ON booking(organizer_key);

CREATE INDEX idx_closure_closing_from ON closure(closing_from);
CREATE INDEX idx_closure_closing_to ON closure(closing_to);
CREATE INDEX idx_closure_organizer_key ON closure(organizer_key);
