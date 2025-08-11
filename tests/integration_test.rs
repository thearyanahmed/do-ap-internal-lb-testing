use std::sync::Mutex;
use actix_web::{test, web, App};
use test0::{AppState, handle_request};

#[tokio::test]
async fn test_http_endpoint_integration() {
    let app_state = web::Data::new(AppState {
        request_count: Mutex::new(0),
        instance_uuid: "integration-test-uuid".to_string(),
    });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(handle_request))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("integration-test-uuid"));
    assert!(body_str.contains("1 requests"));

    let req2 = test::TestRequest::get().uri("/").to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2 = test::read_body(resp2).await;
    let body2_str = std::str::from_utf8(&body2).unwrap();
    
    assert!(body2_str.contains("2 requests"));
}