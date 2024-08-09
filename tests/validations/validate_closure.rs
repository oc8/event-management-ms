use crate::setup_test_context;
use event_protos::event::v1::closure_service_client::ClosureServiceClient;
use event_protos::event::v1::{
    CreateClosureRequest, DeleteClosureRequest, Filters, ListClosuresRequest, UpdateClosureRequest,
};

//
// validations create closure tests
//
#[tokio::test]
async fn create_closure_invalid_dates() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_closure_invalid_dates", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(CreateClosureRequest {
        closing_from: "invalid-date".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    match client.create_closure(request).await {
        Ok(_) => panic!("Expected from date error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    let request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T12:00:00Z".to_string(),
        closing_to: "invalid-date".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    match client.create_closure(request).await {
        Ok(_) => panic!("Expected to date error"),
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
async fn create_closure_invalid_date_range() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_closure_invalid_date_range", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-27T12:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    match client.create_closure(request).await {
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
async fn create_closure_invalid_organizer_key() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_closure_invalid_organizer_key", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "".to_string(),
    });

    match client.create_closure(request).await {
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
// validations update closure tests
//
#[tokio::test]
async fn update_closure_invalid_dates() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_closure_invalid_dates", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(UpdateClosureRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        closing_from: "invalid-date".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
    });

    match client.update_closure(request).await {
        Ok(_) => panic!("Expected from date error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    let request = tonic::Request::new(UpdateClosureRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        closing_from: "2024-05-26T12:00:00Z".to_string(),
        closing_to: "invalid-date".to_string(),
    });

    match client.update_closure(request).await {
        Ok(_) => panic!("Expected to date error"),
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
async fn update_closure_invalid_date_range() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_closure_invalid_date_range", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(UpdateClosureRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        closing_from: "2024-05-27T12:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
    });

    match client.update_closure(request).await {
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
async fn update_closure_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_closure_invalid_id", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(UpdateClosureRequest {
        id: "".to_string(),
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
    });

    match client.update_closure(request).await {
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
// validations delete closure tests
//
#[tokio::test]
async fn delete_closure_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_closure_invalid_id", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(DeleteClosureRequest { id: "".to_string() });

    match client.delete_closure(request).await {
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
// validations list closures tests
//
#[tokio::test]
async fn list_closures_invalid_organizer_key() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("list_closures_invalid_organizer_key", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(ListClosuresRequest {
        filters: Some(Filters {
            organizer_key: "".to_string(),
            ..Default::default()
        }),
    });

    match client.list_closures(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    let request = tonic::Request::new(ListClosuresRequest {
        filters: Some(Filters {
            organizer_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            ..Default::default()
        }),
    });

    match client.list_closures(request).await {
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
