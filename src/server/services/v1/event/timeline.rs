use chrono::{NaiveDateTime};
use rrule::{RRuleSet};
use crate::{add_time_to_datetime, format_datetime, naive_datetime_to_rrule_datetime};
use crate::server::services::v1::booking::booking_model::Booking;
use crate::server::services::v1::closure::closure_model::Closure;
use crate::server::services::v1::event::event_model::{Event, EventStatus};
use protos::event::v1::SlotStatus;
use crate::errors::{ApiError};

pub struct Timeline {
    pub events: Vec<Event>,
    pub closures: Vec<Closure>,
    pub bookings: Vec<Booking>,
}

impl Timeline {
    pub fn new(events: Vec<Event>, closures: Vec<Closure>, bookings: Vec<Booking>) -> Timeline {
        Timeline {
            events,
            closures,
            bookings,
        }
    }

    /// Generate events by recurrence rule within the given time range
    fn generate_events_by_rrule(&self, event: &Event, start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<Event>, ApiError> {
        if let Some(recurrence_rule) = &event.recurrence_rule {
            let recurrence_rule = format!("DTSTART:{}\nRRULE:{}", format_datetime(event.start_time), recurrence_rule);
            let rrule = recurrence_rule.parse::<RRuleSet>()?;

            let after = naive_datetime_to_rrule_datetime(start).unwrap();
            let before = naive_datetime_to_rrule_datetime(end).unwrap();
            let rrule = rrule.after(after).before(before);

            let events = rrule.all(100).dates.iter().map(|d| {
                let mut ve: Event = event.clone();

                ve.end_time = d.naive_utc() + (ve.end_time - ve.start_time);
                ve.start_time = d.naive_utc();

                let mut event_slots = event.clone().slots.unwrap();
                if event.slots.is_some() {
                    event_slots.iter_mut().for_each(|slot| {
                        let slot_start = add_time_to_datetime(ve.start_time, slot.start_time);
                        let slot_end = add_time_to_datetime(ve.end_time, slot.end_time);

                        if event.status == EventStatus::Canceled || event.status == EventStatus::Disabled {
                            slot.status = SlotStatus::Closed;
                            return
                        }

                        log::debug!("closures: {:?}", self.closures);

                        // Check if the slot is within the closure
                        let is_closed = self.closures.iter().any(|closure| {
                            let closure_start = closure.closing_from;
                            let closure_end = closure.closing_to;
                            log::debug!("closure_start: {}, closure_end: {}, closed: {}", closure.closing_from, closure.closing_to, slot_start >= closure_start && slot_end <= closure_end);
                            slot_start >= closure_start && slot_end <= closure_end
                        });

                        // Check if slot is fully booked
                        if !is_closed {
                            if let Some(booking) = self.bookings.iter().find(|b| b.slot_id == slot.id) {
                                let booking_start = booking.date_time;
                                if booking_start >= slot_start && booking_start <= slot_end {
                                    slot.status = SlotStatus::Full;
                                }
                            }
                        } else {
                            slot.status = SlotStatus::Closed;
                        }
                    });
                }

                ve.slots = Some(event_slots.clone());

                let is_closed = self.closures.iter().any(|closure| {
                    let closure_start = closure.closing_from;
                    let closure_end = closure.closing_to;
                    ve.start_time >= closure_start && ve.end_time <= closure_end
                });

                if is_closed {
                    ve.status = EventStatus::Closed;
                }

                ve
            }).collect();

            Ok(events)
        } else {
            Ok(vec![event.clone()])
        }
    }

    /// Return all events that are included in the given time range
    pub fn included(&mut self, start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<Event>, ApiError> {
        log::debug!("Start: {}, End: {}", start, end);

        let mut events: Vec<Event> = self.events.iter().flat_map(|e| {
            self.generate_events_by_rrule(e, start, end).into_iter().flatten()
        }).collect();

        events.sort_by_key(|e| e.start_time);

        for i in 0..events.len() {
            for j in (i + 1)..events.len() {
                if events[i].end_time > events[j].start_time {
                    events[i].overlap = true;
                    events[j].overlap = true;
                }
            }
        }

        Ok(events)
    }
}
