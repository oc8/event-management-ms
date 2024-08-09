use crate::setup_test_context;
use event_protos::event::v1::closure_service_client::ClosureServiceClient;
use event_protos::event::v1::{
    CreateClosureRequest, DeleteClosureRequest, Filters, ListClosuresRequest, UpdateClosureRequest,
};

//
// Create closure tests
//
#[tokio::test]
async fn create_closure() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("create_closure", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    client.create_closure(request).await?;
    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// Update closure tests
//
#[tokio::test]
async fn update_closure() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_closure", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let create_request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    let create_response = client.create_closure(create_request).await.unwrap();
    let closure_id = create_response.into_inner().closure.unwrap().id;

    let update_request = tonic::Request::new(UpdateClosureRequest {
        id: closure_id,
        closing_from: "2024-05-27T09:00:00Z".to_string(),
        closing_to: "2024-05-27T12:00:00Z".to_string(),
    });

    client.update_closure(update_request).await?;
    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn update_closure_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("update_closure_not_found", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let request = tonic::Request::new(UpdateClosureRequest {
        id: "7454c93b-5468-4658-91c2-f4daf4ba60fa".to_string(),
        closing_from: "2024-05-27T09:00:00Z".to_string(),
        closing_to: "2024-05-27T12:00:00Z".to_string(),
    });

    match client.update_closure(request).await {
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

//
// Delete closure tests
//
#[tokio::test]
async fn delete_closure() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("delete_closure", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let create_request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    let create_response = client.create_closure(create_request).await.unwrap();
    let closure_id = create_response.into_inner().closure.unwrap().id;

    let delete_request = tonic::Request::new(DeleteClosureRequest { id: closure_id });

    let delete_response = client.delete_closure(delete_request).await.unwrap();
    assert_eq!(
        delete_response.into_inner().message,
        "Closure deleted successfully"
    );

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

//
// List closures tests
//
#[tokio::test]
async fn list_closures() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("list_closures", 50200).await;
    let mut client = ClosureServiceClient::connect(ctx.url.clone())
        .await
        .unwrap();

    let create_request = tonic::Request::new(CreateClosureRequest {
        closing_from: "2024-05-26T09:00:00Z".to_string(),
        closing_to: "2024-05-26T12:00:00Z".to_string(),
        organizer_key: "test-organizer".to_string(),
    });

    client.create_closure(create_request).await.unwrap();

    let list_request = tonic::Request::new(ListClosuresRequest {
        filters: Some(Filters {
            organizer_key: "test-organizer".to_string(),
            from: "2024-05-26".to_string(),
            to: "2024-06-26".to_string(),
            ..Default::default()
        }),
    });

    let list_response = client.list_closures(list_request).await.unwrap();
    assert!(!list_response.into_inner().closures.is_empty());

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}
