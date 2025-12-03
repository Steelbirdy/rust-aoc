use crate::{samples, Sample};
use crate::utils::Direction;

type Input = Vec<(Direction, i32)>;

const PART1_CHECKS: &[Sample] = samples![
    r"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
L82" => 3,
];

const PART2_CHECKS: &[Sample] = samples![
    r"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82" => 6,
    r"R1000" => 10,
];

pub fn generator(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let mut chars = line.trim().chars();
            let (first, rest) = (chars.next().unwrap(), chars.as_str());
            (Direction::from_char(first).unwrap(), rest.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut password = 0;
    let mut position = 50;

    for &(direction, count) in input {
        position = match direction {
            Direction::Left => (position - count).rem_euclid(100),
            Direction::Right => (position + count).rem_euclid(100),
            _ => unreachable!(),
        };

        if position == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &Input) -> u64 {
    let mut password = 0;
    let mut position = 50;

    for &(direction, count) in input {
        let mut distance = match direction {
            Direction::Left => position,
            Direction::Right => 100 - position,
            _ => unreachable!(),
        };

        if distance == 0 {
            distance = 100;
        }

        if count >= distance {
            password += 1 + (count - distance) / 100;
        }

        position = match direction {
            Direction::Left => (position - count).rem_euclid(100),
            Direction::Right => (position + count).rem_euclid(100),
            _ => unreachable!(),
        };
    }

    password as u64
}

crate::solution!(Input, 2025, 1, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
