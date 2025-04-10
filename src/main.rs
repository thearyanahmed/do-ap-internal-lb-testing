use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;
use uuid::Uuid;

struct AppState {
    request_count: Mutex<u32>,
    instance_uuid: String,
}

async fn handle_request(data: web::Data<AppState>) -> impl Responder {
    let instance_uuid = &data.instance_uuid;

    let mut count = data.request_count.lock().unwrap();
    *count += 1;

    log::info!("[{}] server has handled {} requests.", instance_uuid, count);
    format!("[{}] server has handled {} requests.", instance_uuid, count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let id = Uuid::new_v4();

    let data = web::Data::new(AppState {
        request_count: Mutex::new(0),
        instance_uuid: id.to_string(),
    });

    log::info!("starting server with UUID: {}", id);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(handle_request))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
