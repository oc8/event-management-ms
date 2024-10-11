use crate::errors::ApiError;
use crate::server::services::v1::booking::booking_model::Booking;
use crate::server::services::v1::closure::closure_model::Closure;
use crate::server::services::v1::event::event_model::{Event, EventStatus, EventType};
use crate::{add_time_to_datetime, format_datetime, naive_datetime_to_rrule_datetime};
use chrono::NaiveDateTime;
use event_protos::event::v1::SlotStatus;
use rrule::RRuleSet;
use std::collections::HashMap;

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
    fn generate_events_by_rrule(
        &self,
        event: &Event,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Event>, ApiError> {
        if let Some(recurrence_rule) = &event.recurrence_rule {
            let recurrence_rule = format!(
                "DTSTART:{}\nRRULE:{}",
                format_datetime(event.start_time),
                recurrence_rule
            );
            let rrule = recurrence_rule.parse::<RRuleSet>()?;

            let after = naive_datetime_to_rrule_datetime(start).unwrap();
            let before = naive_datetime_to_rrule_datetime(end).unwrap();
            let rrule = rrule.after(after).before(before);

            let max_gen_events: u16 = std::env::var("MAX_GEN_EVENTS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100);

            let event_bookings: Vec<&Booking> = self.bookings.iter().filter(|b| b.event_id == event.id).collect();
            let closures = &self.closures;

            let events = rrule
                .all(max_gen_events)
                .dates
                .iter()
                .map(|d| {
                    let mut ve = event.clone();

                    ve.end_time = d.naive_utc() + (ve.end_time - ve.start_time);
                    ve.start_time = d.naive_utc();

                    if let Some(mut event_slots) = ve.slots.take() {
                        event_slots.iter_mut().for_each(|slot| {
                            let slot_start = add_time_to_datetime(ve.start_time, slot.start_time);
                            let slot_end = add_time_to_datetime(ve.end_time, slot.end_time);

                            if ve.status == EventStatus::Canceled || ve.status == EventStatus::Disabled {
                                slot.status = SlotStatus::Closed;
                                return;
                            }

                            // Check if the slot is within any closure
                            let is_closed = closures.iter().any(|closure| {
                                let closure_start = closure.closing_from;
                                let closure_end = closure.closing_to;
                                slot_start >= closure_start && slot_end <= closure_end
                            });

                            if !is_closed {
                                // Check if slot is fully booked
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
                        ve.slots = Some(event_slots);
                    }

                    let is_closed = closures.iter().any(|closure| {
                        let closure_start = closure.closing_from;
                        let closure_end = closure.closing_to;
                        ve.start_time >= closure_start && ve.end_time <= closure_end
                    });

                    if is_closed {
                        ve.status = EventStatus::Closed;
                    } else if let Some(capacity) = ve.capacity {
                        ve.capacity = Some(capacity - event_bookings.len() as i32);
                    }

                    ve
                })
                .collect();

            Ok(events)
        } else {
            Ok(vec![event.clone()])
        }
    }

    /// Return all events that are included in the given time range
    pub fn included(
        &mut self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<Event>, ApiError> {
        log::debug!("Start: {}, End: {}", start, end);

        let mut events: Vec<Event> = self
            .events
            .iter()
            .flat_map(|e| self.generate_events_by_rrule(e, start, end).unwrap_or_default())
            .collect();

        events.sort_by_key(|e| e.start_time);

        let mut open_intervals: Vec<&mut Event> = Vec::new();
        for event in events.iter_mut() {
            open_intervals.retain(|e| e.end_time > event.start_time);

            if !open_intervals.is_empty() {
                event.overlap = true;
                for e in open_intervals.iter_mut() {
                    e.overlap = true;
                }
            }

            open_intervals.push(event);
        }

        Ok(events)
    }

}
