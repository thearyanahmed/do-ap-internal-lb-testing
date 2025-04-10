use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    request_count: Mutex<u32>,
}

async fn handle_request(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.request_count.lock().unwrap();
    *count += 1;
    format!("This server has handled {} requests.", count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        request_count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
