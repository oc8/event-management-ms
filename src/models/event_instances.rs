use diesel::dsl::IntervalDsl;
use rrule::RRuleSet;
use poc_booking_ms::{add_time_to_datetime, format_datetime, microseconds_to_naive_time};
use protos::booking::v1::{EventStatus, EventType};
use crate::models::closure::Closure;
use crate::models::event::{Event, EventWithSlots};
use crate::models::slot::Slot;

// EventInstances is used to generate virtual events based on the recurrence rule
pub struct EventInstances {
    pub event: Event,
    pub slots: Vec<Slot>,
    pub closures: Vec<Closure>,
}

impl EventInstances {
    pub fn new(event: Event, slots: Vec<Slot>, closures: Vec<Closure>) -> EventInstances {
        EventInstances {
            event,
            slots,
            closures,
        }
    }

    pub fn create_active_virtual_events(event: &Event, slots: &Vec<Slot>, closures: &Vec<Closure>) -> Vec<EventWithSlots> {
        let mut virtual_events = Vec::new();

        if let Some(recurrence_rule) = &event.recurrence_rule {
            let recurrence_rule = format!("DTSTART:{}\nRRULE:{}", format_datetime(event.start_time), recurrence_rule);
            let recurrence = recurrence_rule.parse::<RRuleSet>();

            match recurrence {
                Ok(rule) => {
                    let occurrences = rule.all(5).dates;

                    for occurrence in occurrences {
                        let mut virtual_event = EventWithSlots::new(event.clone(), Vec::new());
                        virtual_event.event.id = event.id;
                        virtual_event.event.start_time = occurrence.naive_utc();
                        virtual_event.event.end_time = occurrence.naive_utc() + (event.end_time - event.start_time);

                        // Filter slots by occurrence and slot time
                        for slot in slots {
                            let slot_start = add_time_to_datetime(
                                virtual_event.event.start_time,
                                microseconds_to_naive_time(IntervalDsl::microseconds(slot.start_time.0).microseconds)
                            );
                            let slot_end = add_time_to_datetime(
                                virtual_event.event.end_time,
                                microseconds_to_naive_time(IntervalDsl::microseconds(slot.end_time.0).microseconds)
                            );

                            // Check if the slot falls within any closure range
                            let slot_within_closure = closures.iter().any(|closure| {
                                let closure_start = closure.closing_from;
                                let closure_end = closure.closing_to;
                                slot_start >= closure_start && slot_end <= closure_end
                            });

                            // Include the slot if it's outside the closure range
                            if !slot_within_closure {
                                virtual_event.slots.push(slot.clone());
                            }
                        }

                        if !virtual_event.slots.is_empty() {
                            virtual_events.push(virtual_event);
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to parse recurrence rule: {}", e);
                }
            }
        } else {
            if !slots.is_empty() {
                virtual_events.push(EventWithSlots::new(event.clone(), slots.clone()));
            }
        }

        virtual_events.sort_by(|a, b| a.event.start_time.cmp(&b.event.start_time));

        virtual_events
    }
}

impl From<EventInstances> for protos::booking::v1::EventInstances {
    fn from(instance: EventInstances) -> Self {
        let mut proto_instances = protos::booking::v1::EventInstances::default();

        let items = EventInstances::create_active_virtual_events(&instance.event, &instance.slots, &instance.closures);

        proto_instances.id = instance.event.id.to_string();
        proto_instances.name = instance.event.name.clone();
        proto_instances.set_status(EventStatus::from_str_name(&instance.event.status).unwrap_or(EventStatus::Unspecified));
        proto_instances.set_event_type(EventType::from_str_name(&instance.event.event_type).unwrap_or(EventType::Event));
        proto_instances.organizer_key = instance.event.organizer_key.clone();
        proto_instances.max_guests = instance.event.max_guests.unwrap_or_default();
        proto_instances.slot_duration = match instance.event.slot_duration {
            Some(interval) => interval.microseconds / 60_000_000,
            None => 0
        };
        proto_instances.items = items.into_iter().map(|item| {
            let mut proto_item = protos::booking::v1::Event::from(item.clone());
            proto_item.slots = item.slots.iter().map(|slot| {
                protos::booking::v1::Slot::from(slot.clone())
            }).collect();
            proto_item
        }).collect();
        proto_instances.created_at = instance.event.created_at.and_utc().timestamp();
        proto_instances.updated_at = instance.event.updated_at.and_utc().timestamp();

        proto_instances
    }
}
