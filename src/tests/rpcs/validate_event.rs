use tokio::sync::oneshot;
use tonic::transport::Server;
use crate::tests::{TestContext};
use futures_util::FutureExt;
use protos::booking::v1::booking_service_client::BookingServiceClient;
use protos::booking::v1::booking_service_server::BookingServiceServer;
use protos::booking::v1::{CreateEventRequest, EventType};

#[tokio::test]
async fn create_basic_event_invalid_start_date() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event_invalid_start_date", "redis://:@localhost:6382", 6061);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BookingServiceServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "bad-start-date".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        timezone: "Europe/Paris".to_string(),
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
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_end_date() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event_invalid_end_date", "redis://:@localhost:6382", 6062);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BookingServiceServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "bad-end-date".to_string(),
        timezone: "Europe/Paris".to_string(),
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
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_timezone() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event_invalid_timezone", "redis://:@localhost:6382", 6063);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BookingServiceServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        timezone: "Europe/bad-tz".to_string(),
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
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_organizer_key() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event_invalid_organizer_key", "redis://:@localhost:6382", 6064);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BookingServiceServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        timezone: "Europe/Paris".to_string(),
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
    Ok(())
}

#[tokio::test]
async fn create_basic_event_invalid_event_type() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event_invalid_event_type", "redis://:@localhost:6382", 6065);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(BookingServiceServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateEventRequest {
        name: "test-event".to_string(),
        start: "2024-05-26T09:00:00".to_string(),
        end: "2024-05-26T12:00:00".to_string(),
        timezone: "Europe/Paris".to_string(),
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
    Ok(())
}