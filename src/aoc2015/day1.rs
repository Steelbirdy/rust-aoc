use crate::{Sample, samples};

type Input = Vec<u8>;

const PART1_CHECKS: &[Sample] = samples![
    "(())" => 0,
    "()()" => 0,
    "(((" => 3,
    "(()(()(" => 3,
    "))(((((" => 3,
];

const PART2_CHECKS: &[Sample] = samples![
    ")" => 1,
    "()())" => 5,
];

pub fn generator(input: &str) -> Input {
    input.as_bytes().to_vec()
}

pub fn part1(input: &Input) -> u64 {
    input.iter().copied().fold(0_i64, |acc, ch| match ch {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => unreachable!(),
    }) as _
}

pub fn part2(input: &Input) -> u64 {
    let mut acc = 0_u64;
    input
        .iter()
        .copied()
        .position(|ch| {
            match ch {
                b'(' => acc += 1,
                b')' if acc != 0 => acc -= 1,
                b')' => return true,
                _ => unreachable!(),
            }
            false
        })
        .unwrap() as u64
        + 1
}

crate::solution!(
    Input,
    2015,
    1,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
