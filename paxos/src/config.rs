use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of nodes in the simulation.
    #[arg(short, long, default_value_t = 3)]
    pub nodes: usize,

    /// Number of rounds.
    #[arg(short, long, default_value_t = 10)]
    pub rounds: usize,
}

pub fn init_logging() {
    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .without_time()
        .with_target(false)
        .init();
}
