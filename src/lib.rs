pub mod aoc2025;
pub mod aoc2016;
pub mod aoc2015;
pub mod macros;
pub mod utils;

use macros::{samples, solution};
use std::any::Any;

pub struct Solution {
    pub day: u32,
    pub generator: fn(&str) -> Box<dyn Any>,
    pub part1: fn(&dyn Any) -> String,
    pub part1_checks: fn(),
    pub part2: fn(&dyn Any) -> String,
    pub part2_checks: fn(),
}

#[derive(Copy, Clone)]
pub struct Sample {
    pub input: &'static str,
    pub expected_output: u64,
}

inventory::collect!(Solution);
