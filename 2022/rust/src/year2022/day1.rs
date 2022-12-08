//use std::any::type_name;
use std::error::Error;

/// https://stackoverflow.com/a/74298652/923542
//fn type_of<T>(_: T) -> &'static str {
//type_name::<T>()
//}

// thanks raui100 @ github
pub fn solve() -> Result<(), Box<dyn Error>> {
    let max: usize = include_str!("day1.input.txt")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.parse::<usize>().unwrap_or_default())
                .sum()
        })
        //.reduce(|a: usize, b: usize| a.max(b))
        .max()
        .ok_or("Could not find max value")?;
    println!("{}", max);
    Ok(())
}
