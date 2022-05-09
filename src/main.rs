use actix_web::dev::Service;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_starter::toolz::actix_cron::run_main_cron;
use actix_web_starter::toolz::scheduler::start_scheduler;
use actix_web_starter::{
    app_state::build_app_state,
    routes::common,
    toolz::utils::setup_core_env,
};
use futures_util::future::FutureExt;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_core_env();
    let app_data = web::Data::new(Mutex::new(build_app_state()));
    run_main_cron(app_data.clone()).await;
    start_scheduler(app_data.clone()).await;
    println!("[ACTIX_WEB_STARTER](init) => {:#?}", &app_data);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap_fn(|req, srv| {
                println!(
                    "\n\n=============> Hi from start. You requested: {}",
                    req.path()
                );
                srv.call(req).map(|res| {
                    println!("\n\n=============> Hi from response");
                    res
                })
            })
            .service(common::hello)
            .service(common::echo)
            .route("/hey", web::get().to(common::hey))
    })
    .bind(("0.0.0.0", 1342))?
    .run()
    .await
}
