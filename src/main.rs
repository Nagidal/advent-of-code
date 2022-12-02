use clap::Parser;

mod year2022;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    puzzle: u8,
}

fn main() {
    let args = Args::parse();
    match args.puzzle {
        1u8 => year2022::day1(),
        2u8 => year2022::day2(),
        _ => println!("Not yet done"),
    }
}
