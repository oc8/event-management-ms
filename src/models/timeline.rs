use chrono::{NaiveDate, NaiveDateTime};
use rrule::{RRuleSet};
use booking_ms::{add_time_to_datetime, format_datetime, naive_datetime_to_rrule_datetime};
use protos::booking::v1::EventType;
use crate::models::closure::Closure;
use crate::models::event::EventWithSlots;

pub struct Timeline {
    pub events: Vec<EventWithSlots>,
    pub closures: Vec<Closure>,
}

impl Timeline {
    pub fn new(events: Vec<EventWithSlots>, closures: Vec<Closure>) -> Timeline {
        Timeline {
            events,
            closures,
        }
    }

    // Generate events by recurrence rule within the given time range
    // If only_active is true, only return events that have at least one active slot
    fn generate_events_by_rrule(&self, event: &EventWithSlots, start: NaiveDateTime, end: NaiveDateTime, only_active: bool) -> Vec<EventWithSlots> {
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
                        };

                        ve.event.end_time = d.naive_utc() + (ve.event.end_time - ve.event.start_time);
                        ve.event.start_time = d.naive_utc();

                        match only_active {
                            true => {
                                ve.slots = event.slots.iter()
                                    .filter(|slot| {
                                        let slot_start = add_time_to_datetime(ve.event.start_time, slot.start_time);
                                        let slot_end = add_time_to_datetime(ve.event.end_time, slot.end_time);

                                        // Check if the slot is within the closure
                                        !self.closures.iter().any(|closure| {
                                            let closure_start = closure.closing_from;
                                            let closure_end = closure.closing_to;
                                            slot_start >= closure_start && slot_end <= closure_end
                                        })
                                    })
                                    .cloned()
                                    .collect();
                            }
                            false => {
                                ve.slots = event.slots.clone();
                            }
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
            vec![]
        }
    }

    // Return all events that are included in the given time range
    // if only_active is true, only return events that have at least one active slot
    // if end is None, it will default to 7 days after the start time
    pub fn included(&self, start: NaiveDateTime, end: Option<NaiveDateTime>, only_active: bool) -> Vec<EventWithSlots> {
        let end = end.unwrap_or(start + chrono::Duration::days(7));

        let mut events: Vec<EventWithSlots> = self.events.iter().flat_map(|e| {
            self.generate_events_by_rrule(e, start, end, only_active)
        }).collect();

        // Filter out events that are not within the given time range
        // if the event is a meeting, it should have at least one slot to be active
        events.retain(|e|
            (e.event.event_type == EventType::as_str_name(&EventType::Meeting) && !e.slots.is_empty()) &&
                (e.event.start_time <= end && e.event.end_time >= start)
        );

        events.sort_by_key(|e| e.event.start_time);

        events
    }

    // return all active events that are included in the given time range
    pub fn on_date(&self, date: NaiveDate) -> Vec<EventWithSlots> {
        self.events.iter().filter(|e| e.event.start_time.date() == date).cloned().collect()
    }
}
