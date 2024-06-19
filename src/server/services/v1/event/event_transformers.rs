// use protos::event::v1::UpdateEventRequest;
// use crate::server::services::v1::event::event_model::EventUpdate;
//
// impl From <UpdateEventRequest> for EventUpdate {
//     fn from(event: UpdateEventRequest) -> Self {
//         EventUpdate {
//             name: event.name,
//             status: None,
//             start_time: None,
//             end_time: None,
//             recurrence_rule: None,
//             timezone: None,
//             canceled_by: None,
//             canceled_at: None,
//             canceled_reason: None,
//             capacity: None,
//             slot_capacity: None,
//         }
//     }
// }