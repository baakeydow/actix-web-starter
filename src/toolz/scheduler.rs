use crate::app_state::AppState;
use actix::prelude::*;
use actix_web::web;
use chrono::Local;
use cron::Schedule;
use std::sync::Mutex;
use std::{str::FromStr, time::Duration};

// process main task with AppState in ref_data
fn process_main_task(sch: &Scheduler) {
    println!("[SCHEDULER]: => =========================>");
    println!("[SCHEDULER]: => =========================>");
    println!(
        "[SCHEDULER] Task event => {:?} - arc_map keys: {:?}",
        Local::now(),
        sch.ref_data
            .lock()
            .unwrap()
            .arc_map
            .lock()
            .unwrap()
            .keys()
            .len()
    );
    let mut_r_data = sch.ref_data.lock().unwrap();
    if mut_r_data.dev_mode == false {
        println!("[ACTIX_CRON]: (BEFORE DELETE) => {:#?}", mut_r_data);
        mut_r_data.arc_map.lock().unwrap().clear();
        println!("[ACTIX_CRON]: ArcMap Cleared !");
    }
    println!("[SCHEDULER]: => =========================>");
    println!("[SCHEDULER]: => =========================>");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

// Define msg
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping {
    pub ref_data: web::Data<Mutex<AppState<'static>>>,
}

// Define actor
pub struct Scheduler {
    pub ref_data: web::Data<Mutex<AppState<'static>>>,
}

// send AppState to scheduler context
pub async fn start_scheduler(shared_data: web::Data<Mutex<AppState<'static>>>) {
    let addr = Scheduler {
        ref_data: shared_data.clone(),
    }
    .start();
    let result = addr
        .send(Ping {
            ref_data: shared_data,
        })
        .await;
    match result {
        Ok(res) => println!("[SCHEDULER] Got result: {}", res.unwrap()),
        Err(err) => println!("[SCHEDULER] Got error: {}", err),
    }
}

// Task Event logic
impl Scheduler {
    fn schedule_task(&self, ctx: &mut Context<Self>) {
        process_main_task(&self);
        ctx.run_later(
            duration_until_next(&self.ref_data.lock().unwrap().scheduler_time[..]),
            move |this, ctx| this.schedule_task(ctx),
        );
    }
}

// Provide Actor implementation for our actor
impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("[SCHEDULER] Actor is alive");

        ctx.run_later(
            duration_until_next(&self.ref_data.lock().unwrap().scheduler_time[..]),
            move |this, ctx| this.schedule_task(ctx),
        );
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("[SCHEDULER] Actor is stopped");
    }
}

impl<'a> Handler<Ping> for Scheduler {
    type Result = Result<bool, std::io::Error>;

    // Save AppState
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.ref_data = msg.ref_data.clone();
        println!(
            "[SCHEDULER] Message received: {:?} - {:?}",
            msg.ref_data.lock().unwrap().arc_map,
            ctx
        );
        Ok(true)
    }
}

pub fn duration_until_next(cron_expression: &str) -> Duration {
    let cron_schedule = Schedule::from_str(cron_expression).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);
    duration_until.to_std().unwrap()
}
