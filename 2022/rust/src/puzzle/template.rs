pub use crate::common::{Puzzle, Solution};
use tracing::info;

impl Puzzle {
    pub fn new() -> Self {
        let data: String = read_input(0).unwrap_or_default();
        Self { data }
    }
}

impl Solution for Puzzle {
    fn solve(&self) -> () {}

    fn part_a(&self) -> Option<String> {
        None
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}
}
