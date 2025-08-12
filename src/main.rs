use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use uuid::Uuid;
use test0::{AppState, handle_request};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let id = Uuid::new_v4();

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    let data = web::Data::new(AppState {
        request_count: Mutex::new(0),
        instance_uuid: id.to_string(),
        app_env: app_env.clone(),
    });

    log::info!("starting {} server with UUID: {}", app_env, id);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(handle_request))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_handle_request() {
        let app_state = web::Data::new(AppState {
            request_count: Mutex::new(0),
            instance_uuid: "test-uuid".to_string(),
            app_env: "test".to_string(),
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
        
        assert!(body_str.contains("test-uuid"));
        assert!(body_str.contains("1 requests"));
    }

    #[actix_rt::test]
    async fn test_request_counter_increments() {
        let app_state = web::Data::new(AppState {
            request_count: Mutex::new(0),
            instance_uuid: "test-uuid".to_string(),
            app_env: "test".to_string(),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/", web::get().to(handle_request))
        ).await;

        for i in 1..=3 {
            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::call_service(&app, req).await;
            let body = test::read_body(resp).await;
            let body_str = std::str::from_utf8(&body).unwrap();
            
            assert!(body_str.contains(&format!("{} requests", i)));
        }
    }
}
