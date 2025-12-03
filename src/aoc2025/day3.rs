use crate::{samples, Sample};

type Input = Vec<Vec<u8>>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![
r"987654321111111
811111111111119
234234234234278
818181911112111" => 3121910778619,
];

pub fn generator(input: &str) -> Input {
    input.lines()
        .map(|line| {
            line.trim().bytes().map(|b| b - b'0').collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input.iter()
        .map(|line| {
            let (max_index, max) = line[..line.len() - 1].iter()
                .enumerate()
                .fold((0, line[0]), |(prev_index, prev_max), (index, &value)| {
                    if value > prev_max { (index, value) } else { (prev_index, prev_max) }
                });
            let second = line[max_index + 1..].iter()
                .max()
                .copied()
                .unwrap();
            (max * 10 + second) as u64
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input.iter()
        .map(|line| {
            let mut line = line.as_slice();
            let mut result = 0_u64;
            for i in (0_usize..12).rev() {
                let (index, digit) = line[..line.len() - i].iter()
                    .enumerate()
                    .fold((0, line[0]), |(prev_index, prev_digit), (index, &value)| {
                        if value > prev_digit { (index, value) } else { (prev_index, prev_digit) }
                    });
                result = result * 10 + digit as u64;
                line = &line[index + 1..];
            }
            result
        })
        .sum()
}

crate::solution!(Input, 2025, 3, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
