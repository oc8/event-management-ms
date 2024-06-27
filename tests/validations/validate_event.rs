use protos::event::v1::{CreateEventRequest, EventType, GetEventRequest};
use protos::event::v1::event_service_client::EventServiceClient;
use crate::setup_test_context;

//
// validations create event tests
//
#[tokio::test]
async fn create_basic_event_invalid_date_range() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event_invalid_date_range", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T08:00:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_start_date() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event_invalid_start_date", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "bad-start-date".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_end_date() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event_invalid_end_date", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "bad-end-date".to_string(),
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_organizer_key() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event_invalid_organizer_key", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        organizer_key: "".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_event_type() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_basic_event_invalid_event_type", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        organizer_key: "".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: 0,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_meeting_event_invalid_slot_duration() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_meeting_event_invalid_slot_duration", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        organizer_key: "".to_string(),
        slot_duration: 0,
        capacity: 15,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn create_meeting_event_invalid_capacity() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_meeting_event_invalid_capacity", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        organizer_key: "".to_string(),
        slot_duration: 15,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Meeting as i32,
    });

    match client.create_event(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// validations get event tests
//
#[tokio::test]
async fn get_event_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("get_event_invalid_id", 50200).await;
    let mut client = EventServiceClient::connect(ctx.url.clone()).await.unwrap();

    let empty_request = tonic::Request::new(GetEventRequest {
        id: "".to_string(),
    });

    match client.get_event(empty_request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    let invalid_request = tonic::Request::new(GetEventRequest {
        id: "invalid".to_string(),
    });

    match client.get_event(invalid_request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}
