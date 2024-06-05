use chrono::{NaiveDateTime};
use diesel::PgConnection;
use rrule::{RRuleSet};
use booking_ms::{add_time_to_datetime, format_datetime, naive_datetime_to_rrule_datetime};
use protos::booking::v1::{SlotStatus as SlotStatusProto, EventType, EventStatus};
use crate::database::PgPooledConnection;
use crate::errors::{errors, format_error};
use crate::models::booking::{Booking, BookingWithSlot};
use crate::models::closure::Closure;
use crate::models::event::EventWithSlots;

pub struct Timeline {
    pub events: Vec<EventWithSlots>,
    pub closures: Vec<Closure>,
    pub bookings: Vec<BookingWithSlot>,
}

impl Timeline {
    pub fn new(events: Vec<EventWithSlots>, closures: Vec<Closure>, bookings: Vec<BookingWithSlot>) -> Timeline {
        Timeline {
            events,
            closures,
            bookings,
        }
    }

    // Generate events by recurrence rule within the given time range
    fn generate_events_by_rrule(&self, mut event: EventWithSlots, start: NaiveDateTime, end: NaiveDateTime) -> Vec<EventWithSlots> {
        if let Some(recurrence_rule) = &event.event.recurrence_rule {
            let recurrence_rule = format!("DTSTART:{}\nRRULE:{}", format_datetime(event.event.start_time), recurrence_rule);
            let rrule = recurrence_rule.parse::<RRuleSet>();

            match rrule {
                Ok(rule) => {
                    let after = naive_datetime_to_rrule_datetime(start).unwrap();
                    let before = naive_datetime_to_rrule_datetime(end).unwrap();
                    let rrule = rule.after(after).before(before);
                    rrule.all(100).dates.iter().map(|d| {
                        let mut ve = EventWithSlots {
                            event: event.event.clone(),
                            slots: Vec::new(),
                            overlap: false,
                        };

                        ve.event.end_time = d.naive_utc() + (ve.event.end_time - ve.event.start_time);
                        ve.event.start_time = d.naive_utc();

                        event.slots.iter_mut().for_each(|slot| {
                            let slot_start = add_time_to_datetime(ve.event.start_time, slot.slot.start_time);
                            let slot_end = add_time_to_datetime(ve.event.end_time, slot.slot.end_time);

                            // Check if the slot is within the closure
                            let is_closed = self.closures.iter().any(|closure| {
                                let closure_start = closure.closing_from;
                                let closure_end = closure.closing_to;
                                slot_start >= closure_start && slot_end <= closure_end
                            });

                            // Check if slot is fully booked
                            if !is_closed {
                                if let Some(booking) = self.bookings.iter().find(|b| b.booking.slot_id == slot.slot.id) {
                                    let booking_start = booking.booking.date_time;
                                    if booking_start >= slot_start && booking_start <= slot_end {
                                        slot.status = SlotStatusProto::Full;
                                    }
                                }
                            } else {
                                slot.status = SlotStatusProto::Closed;
                            }
                        });

                        ve.slots = event.slots.clone();

                        let is_closed = self.closures.iter().any(|closure| {
                            let closure_start = closure.closing_from;
                            let closure_end = closure.closing_to;
                            ve.event.start_time >= closure_start && ve.event.end_time <= closure_end
                        });

                        if is_closed {
                            ve.event.status = EventStatus::Closed.as_str_name().parse().unwrap();
                        }

                        ve
                    }).collect()
                },
                Err(e) => {
                    // TODO: Maybe return an error instead of an empty vector
                    log::error!("Error parsing rrule: {}", e);
                    vec![]
                }
            }
        } else {
            vec![event.clone()]
        }
    }

    // Return all events that are included in the given time range
    // if only_active is true, only return events that have at least one active slot
    // if end is None, it will default to 7 days after the start time
    pub fn included(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Vec<EventWithSlots> {
        log::debug!("Start: {}, End: {}", start, end);

        let mut events: Vec<EventWithSlots> = self.events.iter().flat_map(|e| {
            self.generate_events_by_rrule(e.clone(), start, end)
        }).collect();

        // Filter out inactive events if only_active is true
        // if the event is a meeting, it should have at least one slot to be active
        events.retain(|e|
            (e.event.event_type == EventType::as_str_name(&EventType::Meeting) && !e.slots.is_empty() ||
            e.event.event_type != EventType::as_str_name(&EventType::Meeting))
        );

        events.sort_by_key(|e| e.event.start_time);

        for i in 0..events.len() {
            for j in (i + 1)..events.len() {
                if events[i].event.end_time > events[j].event.start_time {
                    events[i].overlap = true;
                    events[j].overlap = true;
                }
            }
        }

        events
    }
}
