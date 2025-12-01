use crate::{Sample, samples};

type Input = (usize, usize);

const PART1_CHECKS: &[Sample] = samples![
    "To continue, please consult the code grid in the manual.  Enter the code at row 3, column 1." => 16080970,
];

const PART2_CHECKS: &[Sample] = samples![];

const FIRST_CODE: u64 = 20151125;

const MULTIPLIER: u64 = 252533;

const DIVISOR: u64 = 33554393;

pub fn generator(input: &str) -> Input {
    prse::parse!(
        input,
        "To continue, please consult the code grid in the manual.  Enter the code at row {}, column {}."
    )
}

pub fn part1(input: &Input) -> u64 {
    let &(row, col) = input;
    let diagonal = row + col - 1;
    let first_index_of_diagonal = crate::utils::nth_triangle_number((diagonal - 1) as u64);
    let index_in_diagonal = col - 1;
    let index = first_index_of_diagonal + index_in_diagonal as u64;
    (FIRST_CODE * crate::utils::modular_pow(MULTIPLIER, index, DIVISOR)) % DIVISOR
}

pub fn part2(_: &Input) -> u64 {
    2015 - 12 - 25
}

crate::solution!(
    Input,
    25,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
