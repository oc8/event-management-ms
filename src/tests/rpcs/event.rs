use tokio::sync::oneshot;
use tonic::transport::Server;
use crate::tests::{TestContext};
use futures_util::FutureExt;
use protos::booking::v1::booking_service_client::BookingServiceClient;
use protos::booking::v1::booking_service_server::BookingServiceServer;
use protos::booking::v1::{CreateEventRequest, EventType};

#[tokio::test]
async fn create_basic_event() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@localhost:5433", "create_basic_event", "redis://:@localhost:6382", 6060);
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
        organizer_key: "test-organizer".to_string(),
        slot_duration: 0,
        capacity: 0,
        slot_capacity: 0,
        recurrence_rule: "".to_string(),
        event_type: EventType::Event as i32,
    });
    client.create_event(request).await?;

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}
