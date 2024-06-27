use event_ms::add_time_to_datetime;
use protos::event::v1::booking_service_client::BookingServiceClient;
use protos::event::v1::{CreateBookingRequest, CreateEventRequest, DeleteBookingRequest, EventType, GetBookingRequest, ListBookingsRequest};
use protos::event::v1::event_service_client::EventServiceClient;
use crate::setup_test_context;

#[tokio::test]
async fn create_booking() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_booking", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();
    let mut event_client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let start = chrono::Utc::now() + chrono::Duration::days(1);
    let end = start + chrono::Duration::hours(4);

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: start.to_rfc3339(),
        end: end.to_rfc3339(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 15,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    let response = event_client.create_event(request).await?;
    let event = response.into_inner().event.unwrap();

    let slot = event.slots.first().unwrap();

    let datetime = add_time_to_datetime(start.naive_utc(), slot.clone().start.unwrap().date_time.parse().unwrap());

    let request = tonic::Request::new(CreateBookingRequest {
        slot_id: slot.id.clone(),
        booking_holder_key: "test-holder-key".to_string(),
        date_time: datetime.to_string() + "+00:00",
        persons: 1,
    });

    let response = client.create_booking(request).await?;
    assert!(response.into_inner().booking.is_some());

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn get_booking() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("get_booking", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();
    let mut event_client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let start = chrono::Utc::now() + chrono::Duration::days(1);
    let end = start + chrono::Duration::hours(4);

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: start.to_rfc3339(),
        end: end.to_rfc3339(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 15,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    let response = event_client.create_event(request).await?;
    let event = response.into_inner().event.unwrap();

    let slot = event.slots.first().unwrap();

    let datetime = add_time_to_datetime(start.naive_utc(), slot.clone().start.unwrap().date_time.parse().unwrap());

    let create_request = tonic::Request::new(CreateBookingRequest {
        slot_id: slot.id.clone(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: datetime.to_string() + "+00:00",
        persons: 1,
    });

    let create_response = client.create_booking(create_request).await.unwrap();
    let booking_id = create_response.into_inner().booking.unwrap().id;

    let get_request = tonic::Request::new(GetBookingRequest {
        id: booking_id,
    });

    let get_response = client.get_booking(get_request).await?;
    assert!(get_response.into_inner().booking.is_some());

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn delete_booking() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_booking", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();
    let mut event_client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let start = chrono::Utc::now() + chrono::Duration::days(1);
    let end = start + chrono::Duration::hours(4);

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: start.to_rfc3339(),
        end: end.to_rfc3339(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 15,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    let response = event_client.create_event(request).await?;
    let event = response.into_inner().event.unwrap();

    let slot = event.slots.first().unwrap();

    let datetime = add_time_to_datetime(start.naive_utc(), slot.clone().start.unwrap().date_time.parse().unwrap());

    let create_request = tonic::Request::new(CreateBookingRequest {
        slot_id: slot.id.clone(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: datetime.to_string() + "+00:00",
        persons: 1,
    });

    let create_response = client.create_booking(create_request).await.unwrap();
    let booking_id = create_response.into_inner().booking.unwrap().id;

    let delete_request = tonic::Request::new(DeleteBookingRequest {
        id: booking_id,
    });

    let delete_response = client.delete_booking(delete_request).await?;
    assert_eq!(delete_response.into_inner().message, "Booking successfully deleted");

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn list_bookings() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("list_bookings", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();
    let mut event_client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let start = chrono::Utc::now() + chrono::Duration::days(1);
    let end = start + chrono::Duration::hours(4);

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: start.to_rfc3339(),
        end: end.to_rfc3339(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 15,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    let response = event_client.create_event(request).await?;
    let event = response.into_inner().event.unwrap();

    let slot = event.slots.first().unwrap();

    let datetime = add_time_to_datetime(start.naive_utc(), slot.clone().start.unwrap().date_time.parse().unwrap());

    let create_request = tonic::Request::new(CreateBookingRequest {
        slot_id: slot.id.clone(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: datetime.to_string() + "+00:00",
        persons: 1,
    });

    client.create_booking(create_request).await.unwrap();

    let list_request = tonic::Request::new(ListBookingsRequest {
        filters: Some(protos::event::v1::Filters {
            organizer_key: "test-organizer".to_string(),
            ..Default::default()
        }),
    });

    let list_response = client.list_bookings(list_request).await?;
    assert!(!list_response.into_inner().bookings.is_empty());

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}
