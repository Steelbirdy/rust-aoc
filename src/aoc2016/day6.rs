use crate::{Sample, samples};
use itertools::Itertools;

type Input = Vec<Vec<char>>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &Input) -> String {
    (0..input[0].len())
        .map(|i| {
            let (most_common, _) = input
                .iter()
                .map(|line| line[i])
                .counts()
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .unwrap();
            most_common
        })
        .collect()
}

pub fn part2(input: &Input) -> String {
    (0..input[0].len())
        .map(|i| {
            let (least_common, _) = input
                .iter()
                .map(|line| line[i])
                .counts()
                .into_iter()
                .min_by_key(|&(_, count)| count)
                .unwrap();
            least_common
        })
        .collect()
}

crate::solution!(
    Input,
    2016,
    6,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
