use clap::Parser;

/// Simple program to greaet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of nodes
    #[arg(short, long, default_value_t = 3)]
    pub nodes: usize,

    /// Number of rounds
    #[arg(short, long, default_value_t = 10)]
    pub rounds: usize,
}
