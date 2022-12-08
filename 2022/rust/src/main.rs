#![feature(iter_array_chunks)]
use color_eyre::eyre::{Report, Result};

mod cli;
mod year2022;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    cli::parse_args()?;
    Ok(())
    //let args = Args::parse();
    //match args.puzzle {
    //1u8 => year2022::day1::solve(),
    //2u8 => year2022::day2::solve(),
    //3u8 => year2022::day3::solve(),
    //4u8 => year2022::day4::solve(),
    //5u8 => year2022::day5::solve(),
    //_ => {
    //println!("Not yet done");
    //Ok(())
    //}
    //}
}
