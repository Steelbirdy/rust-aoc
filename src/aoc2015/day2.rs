use crate::{Sample, samples};

type Input = Vec<(u64, u64, u64)>;

const PART1_CHECKS: &[Sample] = samples![
    "2x3x4" => 58,
    "1x1x10" => 43,
];

const PART2_CHECKS: &[Sample] = samples![
    "2x3x4" => 34,
    "1x1x10" => 14,
];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('x');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .copied()
        .map(|(l, w, h)| {
            let lw = l * w;
            let lh = l * h;
            let wh = w * h;
            2 * (l * w + l * h + w * h) + lw.min(lh).min(wh)
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .iter()
        .copied()
        .map(|(l, w, h)| {
            let mut sides = [l, w, h];
            sides.sort();
            2 * (sides[0] + sides[1]) + l * w * h
        })
        .sum()
}

crate::solution!(
    Input,
    2015,
    2,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
