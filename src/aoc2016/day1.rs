use crate::{Sample, samples, utils::Direction};
use std::collections::HashSet;

type Input = Vec<(Direction, u32)>;

const PART1_CHECKS: &[Sample] = samples![
    "R2, L3" => 5,
    "R2, R2, R2" => 2,
    "R5, L5, R5, R3" => 12,
];

const PART2_CHECKS: &[Sample] = samples![
    "R8, R4, R4, R8" => 4,
];

pub fn generator(input: &str) -> Input {
    input
        .split(", ")
        .map(|s| {
            let mut ch = s.chars();
            (
                Direction::from_char(ch.next().unwrap()).unwrap(),
                ch.as_str().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;

    for &(d, steps) in input {
        direction.rotate(d);
        let (dx, dy) = direction.deltas();

        x += (steps as i32) * dx;
        y += (steps as i32) * dy;
    }

    (x.abs() + y.abs()) as u64
}

pub fn part2(input: &Input) -> u64 {
    let mut visited = HashSet::new();

    let mut direction = Direction::Up;
    let mut x = 0_i32;
    let mut y = 0_i32;

    for &(d, steps) in input {
        direction.rotate(d);
        let (dx, dy) = direction.deltas();

        for step in 0..steps as i32 {
            let sx = x + step * dx;
            let sy = y + step * dy;
            if !visited.insert((sx, sy)) {
                return (sx.abs() + sy.abs()) as u64;
            }
        }

        x += (steps as i32) * dx;
        y += (steps as i32) * dy;
    }

    panic!("no location visited twice")
}

crate::solution!(
    Input,
    2016,
    1,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
