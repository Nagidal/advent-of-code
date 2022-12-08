#![feature(iter_array_chunks)]
use crate::common::Solution;
use color_eyre::eyre::{Report, Result};
use std::env;
use tracing::{instrument, trace, warn};
use tracing_subscriber::{fmt, EnvFilter};

mod common;
mod puzzle;

fn install_tracing() {
    fmt()
        .without_time()
        .with_line_number(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("trace"))
                .unwrap(),
        )
        .init();
}

#[instrument]
fn main() -> Result<(), Report> {
    install_tracing();
    color_eyre::install()?;
    let day: u8 = env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
    trace!(day);
    match day {
        0 => warn!("Specify a puzzle to solve, e.g:  cargo run -- 1"),
        _ => puzzle::day01::Puzzle::new().solve(),
    }
    Ok(())
}
