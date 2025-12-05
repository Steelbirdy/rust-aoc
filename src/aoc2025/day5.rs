use crate::{Sample, samples};
use std::ops::RangeInclusive;

type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![
r#"3-5
10-14
16-20
12-18

1"# => 14,
];

pub fn generator(input: &str) -> Input {
    let (first, second) = input.split_once("\n\n").unwrap();
    let first = first
        .lines()
        .map(|line| {
            let (lo, hi) = line.split_once('-').unwrap();
            lo.parse().unwrap()..=hi.parse().unwrap()
        })
        .collect();
    let second = second.lines().map(|line| line.parse().unwrap()).collect();
    (first, second)
}

pub fn part1(input: &Input) -> u64 {
    let (fresh_ingredients, available_ingredients) = input;

    let ranges = crate::utils::merge_ranges(fresh_ingredients.iter().cloned());
    available_ingredients
        .iter()
        .filter(|&ingredient| {
            match ranges.binary_search_by_key(ingredient, |range| *range.start()) {
                Ok(_) => true,
                Err(0) => false,
                Err(i) => ranges[i - 1].contains(ingredient),
            }
        })
        .count() as u64
}

pub fn part2(input: &Input) -> u64 {
    let (fresh_ingredients, _) = input;
    let ranges = crate::utils::merge_ranges(fresh_ingredients.iter().cloned());
    ranges.into_iter().map(|range| range.count() as u64).sum()
}

crate::solution!(
    Input,
    2025,
    5,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
