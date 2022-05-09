use crate::{toolz::utils::get_ip_addr, app_state::AppState};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn hey(req: HttpRequest, data: web::Data<Mutex<AppState<'_>>>) -> impl Responder {
    let mut state = data.lock().unwrap();
    let ip = get_ip_addr(req);
    let key = Box::leak(format!("hey-{ip}").into_boxed_str());
    state.update_endpoint_count(key);
    let count = state.get_endpoint_count(key);
    HttpResponse::Ok().body(format!("Hey {ip} welcome, endpoint requested: {count}"))
}

#[get("/")]
pub async fn hello(req: HttpRequest, data: web::Data<Mutex<AppState<'_>>>) -> impl Responder {
    let mut state = data.lock().unwrap();
    let ip = get_ip_addr(req);
    let key = Box::leak(format!("root-{ip}").into_boxed_str());
    let app_name = &state.app_name.clone();
    state.update_endpoint_count(key);
    let count = state.get_endpoint_count(key);
    HttpResponse::Ok().body(format!(
        "Hey {ip} welcome to {app_name}, endpoint requested: {count}"
    ))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
