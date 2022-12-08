use clap::Parser;
use color_eyre::eyre::Report;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Day to run
    day: Option<String>,
    // Year
    #[arg(default_value_t = 2022)]
    year: u16,
}

pub fn parse_args() -> Result<(), Report> {
    let cli = Args::parse();
    Ok(())
}
