use protos::booking::v1::booking_service_client::BookingServiceClient;
use protos::booking::v1::{CreateBookingRequest, DeleteBookingRequest, GetBookingRequest, ListBookingsRequest};
use crate::tests::setup_test_context;

#[tokio::test]
async fn create_booking_invalid_slot_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_booking_invalid_slot_id", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateBookingRequest {
        slot_id: "invalid-slot-id".to_string(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: "2024-05-26T09:00:00Z".to_string(),
        persons: 1,
    });

    match client.create_booking(request).await {
        Ok(_) => panic!("Expected slot ID error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn create_booking_invalid_datetime() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_booking_invalid_datetime", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateBookingRequest {
        slot_id: "valid-slot-id".to_string(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: "invalid-datetime".to_string(),
        persons: 1,
    });

    match client.create_booking(request).await {
        Ok(_) => panic!("Expected datetime error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn create_booking_invalid_persons_number() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_booking_invalid_persons_number", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(CreateBookingRequest {
        slot_id: "valid-slot-id".to_string(),
        booking_holder_key: "valid-holder-key".to_string(),
        date_time: "2024-05-26T09:00:00Z".to_string(),
        persons: 10001,
    });

    match client.create_booking(request).await {
        Ok(_) => panic!("Expected persons number error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn get_booking_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("get_booking_invalid_id", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GetBookingRequest {
        id: "invalid-id".to_string(),
    });

    match client.get_booking(request).await {
        Ok(_) => panic!("Expected event ID error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn delete_booking_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_booking_invalid_id", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(DeleteBookingRequest {
        id: "invalid-id".to_string(),
    });

    match client.delete_booking(request).await {
        Ok(_) => panic!("Expected event ID error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn list_bookings_invalid_filters() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("list_bookings_invalid_filters", 50200).await;
    let mut client = BookingServiceClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(ListBookingsRequest {
        filters: Some(protos::booking::v1::Filters {
            organizer_key: "".to_string(),
            ..Default::default()
        }),
    });

    match client.list_bookings(request).await {
        Ok(_) => panic!("Expected filters error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}
