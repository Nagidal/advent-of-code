use color_eyre::Result;
use std::fs;
use tracing::info;

pub struct Puzzle {
    pub data: String,
}

pub trait Solution {
    fn solve(&self) -> () {
        let sol1 = self
            .part_a()
            .unwrap_or("Error in solving part_a".to_string());
        let sol2 = self
            .part_b()
            .unwrap_or("Error in solving part_b".to_string());
        info!(sol1, sol2);
    }

    fn part_a(&self) -> Option<String> {
        None
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}

pub fn read_input(n: u8) -> Result<String, std::io::Error> {
    let number = if n < 10 {
        format!("0{n}")
    } else {
        n.to_string()
    };
    fs::read_to_string(format!("./input/day{}.input.txt", number))
}
