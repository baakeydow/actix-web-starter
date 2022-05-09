use crate::app_state::AppState;
use actix_web::web;
use actix_web::HttpRequest;
use chrono::DateTime;
use chrono::{Local, Timelike};
use std::sync::Mutex;

pub fn format_datetime(date_to_fmt: DateTime<Local>) -> String {
    let (is_pm, hour) = date_to_fmt.hour12();
    let min = date_to_fmt.minute();
    let sec = date_to_fmt.second();
    let am_pm = if is_pm { "PM" } else { "AM" };
    let str = format!("{hour}:{min}:{sec} {am_pm}");
    str
}

pub fn display_cron_debug(shared_data: &web::Data<Mutex<AppState<'static>>>) {
    println!("[ACTIX_CRON]: => =========================>");
    println!("[ACTIX_CRON]: => {}", format_datetime(Local::now()));
    let mut_r_data = &mut shared_data.lock().unwrap();
    if mut_r_data.dev_mode {
        println!("[ACTIX_CRON]: => {:#?}", mut_r_data);
    }
    for (key, value) in &*mut_r_data.arc_map.lock().unwrap() {
        println!("[ACTIX_CRON]: {} => {:?}", key, value);
    }
    println!("[ACTIX_CRON]: => =========================>");
    println!("-------------------------------------------");
}

pub fn setup_core_env() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

pub fn get_ip_addr(req: HttpRequest) -> String {
    let ip = match req.connection_info().realip_remote_addr() {
        Some(val) => val.to_string(),
        None => panic!("Failed to retrieve ip"),
    };
    ip
}
