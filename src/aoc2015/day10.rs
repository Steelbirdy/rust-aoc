use crate::{Sample, samples};

type Input = Vec<(u8, u8)>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    let mut current = None;
    let mut current_count = 0_u8;
    let mut result = Vec::new();
    for b in input.bytes() {
        if Some(b) == current {
            current_count += 1;
            continue;
        }
        if let Some(current) = current.take() {
            result.push((current, current_count));
        }
        current = Some(b - b'0');
        current_count = 1;
    }

    if let Some(current) = current.take() {
        result.push((current, current_count));
    }

    result
}

fn merge(digits: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let mut result = Vec::new();

    let mut current = None;
    let mut current_count = 0_u8;
    for &(digit, count) in digits {
        if Some(digit) == current {
            current_count += count;
            continue;
        }

        if let Some(current) = current.take() {
            result.push((current_count, 1));
            result.push((current, 1));
        }
        current = Some(digit);
        current_count = count;
    }

    if let Some(current) = current.take() {
        result.push((current_count, 1));
        result.push((current, 1));
    }

    result
}

pub fn part1(input: &Input) -> u64 {
    let mut input = input.to_vec();
    for _ in 0..40 {
        input = merge(&input);
    }
    input.iter().map(|&(_, count)| count as u64).sum::<u64>()
}

pub fn part2(input: &Input) -> u64 {
    let mut input = input.to_vec();
    for _ in 0..50 {
        input = merge(&input);
    }
    input.iter().map(|&(_, count)| count as u64).sum::<u64>()
}

crate::solution!(
    Input,
    2015,
    10,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
