pub use crate::common::{read_input, Puzzle, Solution};
use tracing::info;

impl Puzzle {
    pub fn new() -> Self {
        let data: String = read_input(1).unwrap_or_default();
        Self { data }
    }
}

impl Solution for Puzzle {
    fn part_a(&self) -> Option<String> {
        info!("Good job");
        Some((&self.data.lines().nth(0)?).to_string())
    }
}
