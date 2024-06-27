use crate::{setup_test_context};
use protos::event::v1::{CancelEventRequest, CreateEventRequest, DeleteEventRequest, Event, EventStatus, EventType, GetEventRequest, UpdateEventRequest};
use protos::event::v1::event_service_client::EventServiceClient;

fn assert_event_fields(expected: &CreateEventRequest, actual: &Event) {
    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.start, actual.start.clone().unwrap().date_time);
    assert_eq!(expected.end, actual.end.clone().unwrap().date_time);
    assert_eq!(expected.organizer_key, actual.organizer_key);
    assert_eq!(expected.slot_duration, actual.slot_duration);
    assert_eq!(expected.capacity, actual.capacity);
    assert_eq!(expected.recurrence_rule, actual.recurrence_rule);
    assert_eq!(expected.event_type, actual.event_type);
}

//
// Create event tests
//
#[tokio::test]
async fn create_basic_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_request = CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T11:00:00+02:00".to_string(),
        end: "2024-05-26T14:00:00+02:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    };
    let mut request = tonic::Request::new(create_request.clone());
    request.metadata_mut().insert("timezone", "Europe/Paris".parse()?);
    let response = client.create_event(request).await?;

    let event = response.into_inner().event.unwrap();

    assert_event_fields(&create_request, &event);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn get_event_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("get_event_not_found", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GetEventRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
    });


    match client.get_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn get_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("get_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_request = CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T11:00:00+02:00".to_string(),
        end: "2024-05-26T14:00:00+02:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    };
    let create_event_request = tonic::Request::new(create_request.clone());
    let resp = client.create_event(create_event_request).await?;
    let event = resp.into_inner().event.unwrap();

    let mut request = tonic::Request::new(GetEventRequest {
        id: event.id.clone()
    });
    request.metadata_mut().insert("timezone", "Europe/Paris".parse()?);


    let resp = client.get_event(request).await?;
    let client = resp.into_inner().event.unwrap();

    assert_eq!(event.id, client.id);
    assert_event_fields(&create_request, &client);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_recurrent_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_recurrent_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_request = CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T11:00:00+02:00".to_string(),
        end: "2024-05-26T14:00:00+02:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "FREQ=WEEKLY;BYDAY=SU".to_string(),
        event_type: EventType::Event as i32,
    };
    let mut request = tonic::Request::new(create_request.clone());
    request.metadata_mut().insert("timezone", "Europe/Paris".parse()?);
    let response = client.create_event(request).await?;

    let event = response.into_inner().event.unwrap();
    assert_event_fields(&create_request, &event);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_meeting_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_meeting_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_request = CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T11:00:00+02:00".to_string(),
        end: "2024-05-26T14:00:00+02:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 15,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "FREQ=WEEKLY;BYDAY=SU".to_string(),
        event_type: EventType::Event as i32,
    };

    let mut request = tonic::Request::new(create_request.clone());
    request.metadata_mut().insert("timezone", "Europe/Paris".parse()?);
    let response = client.create_event(request).await?;

    let event = response.into_inner().event.unwrap();
    assert_event_fields(&create_request, &event);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// Update Event tests
//

#[tokio::test]
async fn update_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_request = CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T11:00:00+02:00".to_string(),
        end: "2024-05-26T14:00:00+02:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    };

    let create_event_request = tonic::Request::new(create_request.clone());
    let create_resp = client.create_event(create_event_request).await?;
    let event = create_resp.into_inner().event.unwrap();

    let mut update_event_request = tonic::Request::new(UpdateEventRequest {
        id: event.id.clone(),
        name: "updated-event".to_string(),
        start: "2024-05-26T12:00:00+02:00".to_string(),
        end: "2024-05-26T13:00:00+02:00".to_string(),
        capacity: 100,
        slot_capacity: 10,
        recurrence_rule: "".to_string(),
    });
    update_event_request.metadata_mut().insert("timezone", "Europe/Paris".parse()?);
    let update_resp = client.update_event(update_event_request).await?;
    let updated_event = update_resp.into_inner().event.unwrap();

    assert_eq!(updated_event.name, "updated-event");
    assert_eq!(updated_event.start.clone().unwrap().date_time, "2024-05-26T12:00:00+02:00");
    assert_eq!(updated_event.end.unwrap().date_time, "2024-05-26T13:00:00+02:00");
    assert_eq!(updated_event.capacity, 100);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn update_event_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_event_not_found", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(UpdateEventRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        name: "non-existent-event".to_string(),
        start: "2024-05-26T10:00:00Z".to_string(),
        end: "2024-05-26T11:00:00Z".to_string(),
        capacity: 100,
        slot_capacity: 10,
        recurrence_rule: "".to_string(),
    });

    match client.update_event(request).await {
        Ok(_) => panic!("Expected error for non-existent event"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// Delete Event tests
//
#[tokio::test]
async fn delete_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_event_request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00Z".to_string(),
        end: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });
    let create_resp = client.create_event(create_event_request).await?;
    let event = create_resp.into_inner().event.unwrap();

    let delete_event_request = tonic::Request::new(DeleteEventRequest {
        id: event.id.clone(),
    });
    let delete_resp = client.delete_event(delete_event_request).await?;
    assert_eq!(delete_resp.into_inner().message, "Event deleted successfully");

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn delete_event_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_event_not_found", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(DeleteEventRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
    });

    match client.delete_event(request).await {
        Ok(_) => panic!("Expected error for non-existent event"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// Cancel Event tests
//
#[tokio::test]
async fn cancel_event() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("cancel_event", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let create_event_request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00Z".to_string(),
        end: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });
    let create_resp = client.create_event(create_event_request).await?;
    let event = create_resp.into_inner().event.unwrap();

    let cancel_event_request = tonic::Request::new(CancelEventRequest {
        id: event.id.clone(),
        canceled_by: "test-user".to_string(),
        reason: "Changed plans".to_string(),
    });
    let cancel_resp = client.cancel_event(cancel_event_request).await?;
    let canceled_event = cancel_resp.into_inner().event.unwrap();

    assert_eq!(canceled_event.status, EventStatus::Canceled as i32);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn cancel_event_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("cancel_event_not_found", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CancelEventRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        canceled_by: "test-user".to_string(),
        reason: "No such event".to_string(),
    });

    match client.cancel_event(request).await {
        Ok(_) => panic!("Expected error for non-existent event"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}
