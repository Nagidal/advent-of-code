use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let data_str = include_str!("day1.input.txt");
    let chunks_str_raw: Vec<&str> = data_str.split("\n\n").collect();
    let chunks_str: Vec<Vec<&str>> = chunks_str_raw
        .iter()
        .map(|s| s.split("\n").collect())
        .collect();
    let groups: Vec<Vec<usize>> = chunks_str
        .iter()
        .map(|v| {
            v.iter()
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .collect()
        })
        .collect();
    let mut sums: Vec<usize> = vec![];
    for group in &groups {
        sums.push(group.iter().sum());
    }
    println!("{}", sums.iter().max().ok_or("Could not find max value")?);
    Ok(())
}
