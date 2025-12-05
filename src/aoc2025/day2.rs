use crate::{Sample, samples};
use rayon::prelude::*;
use std::ops::RangeInclusive;

type Input = Vec<RangeInclusive<u64>>;

const PART1_CHECKS: &[Sample] = samples![
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862" => 1227775554,
];

const PART2_CHECKS: &[Sample] = samples![
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124" => 4174379265,
];

pub fn generator(input: &str) -> Input {
    input
        .split(',')
        .map(|range| {
            let (low, high) = range.split_once('-').unwrap();
            low.parse().unwrap()..=high.parse().unwrap()
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input
        .par_iter()
        .cloned()
        .flatten()
        .filter(|&n| {
            let digits = n.ilog10() + 1;
            let ten_pow = 10_u64.pow(digits / 2);
            digits.is_multiple_of(2) && n % ten_pow == n / ten_pow
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .par_iter()
        .cloned()
        .flatten()
        .filter(|&n| {
            let digits = n.ilog10() + 1;
            (2..=digits).filter(|&c| digits.is_multiple_of(c)).any(|c| {
                let ten_pow = 10_u64.pow(digits / c);
                let seq = n % ten_pow;
                let mut denominator = 1;
                (0..c - 1).all(|_| {
                    denominator *= ten_pow;
                    n / denominator % ten_pow == seq
                })
            })
        })
        .sum()
}

crate::solution!(
    Input,
    2025,
    2,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
