use crate::{Sample, samples};
use rayon::prelude::*;

type Input = String;

const PART1_CHECKS: &[Sample] = samples![
    "abcdef" => 609043,
    "pqrstuv" => 1048970,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> u64 {
    (0..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&n| {
            let digest = md5::compute(format!("{input}{n}"));
            digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0
        })
        .unwrap() as _
}

pub fn part2(input: &Input) -> u64 {
    (0..u32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&n| {
            let digest = md5::compute(format!("{input}{n}"));
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        })
        .unwrap() as _
}

crate::solution!(
    Input,
    4,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
