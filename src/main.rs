use clap::Parser;
use std::error::Error;

mod year2022;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    puzzle: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.puzzle {
        1u8 => year2022::day1::solve(),
        2u8 => year2022::day2::solve(),
        3u8 => year2022::day3::solve(),
        4u8 => year2022::day4::solve(),
        5u8 => year2022::day5::solve(),
        _ => {
            println!("Not yet done");
            Ok(())
        }
    }
}
