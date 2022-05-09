use clap::Parser;

/// ACTIX-WEB-STARTER
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CoreArgs {
    /// Dev mode
    #[clap(short, long)]
    pub dev: bool,

    /// Main application tick
    #[clap(short, long, default_value_t = 10)]
    pub cron_time: u64,

    /// Scheduler tick
    #[clap(short, long, default_value = "0 * * * * * *")]
    pub sch_time: String,

    /// Max count per protected endpoint
    #[clap(short, long, default_value_t = 100)]
    pub max_endpoint_count: u64,
}


