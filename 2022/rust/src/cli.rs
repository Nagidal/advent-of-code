use clap::Parser;
use color_eyre::eyre::Report;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Day to run
    day: Option<String>,
}

pub fn parse_args() -> Result<(), Report> {
    let cli = Args::parse();
    Ok(())
}
