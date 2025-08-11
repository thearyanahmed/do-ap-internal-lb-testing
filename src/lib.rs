use actix_web::{web, Responder};
use std::sync::Mutex;

pub struct AppState {
    pub request_count: Mutex<u32>,
    pub instance_uuid: String,
}

pub async fn handle_request(data: web::Data<AppState>) -> impl Responder {
    let instance_uuid = &data.instance_uuid;

    let mut count = data.request_count.lock().unwrap();
    *count += 1;

    log::info!("[{}] server has handled {} requests.", instance_uuid, count);
    format!("[{}] server has handled {} requests.", instance_uuid, count)
}