use crate::{Sample, samples};
use std::collections::HashSet;

type Input = Vec<(isize, isize)>;

const PART1_CHECKS: &[Sample] = samples![
    ">" => 2,
    "^>v<" => 4,
    "^v^v^v^v^v" => 2,
];

const PART2_CHECKS: &[Sample] = samples![
    "^v" => 3,
    "^>v<" => 3,
    "^v^v^v^v^v" => 11,
];

pub fn generator(input: &str) -> Input {
    input
        .chars()
        .map(|ch| match ch {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut visited = HashSet::new();
    let (mut x, mut y) = (0_isize, 0_isize);
    for &(dx, dy) in input {
        visited.insert((x, y));
        x += dx;
        y += dy;
    }
    visited.insert((x, y));

    visited.len() as _
}

pub fn part2(input: &Input) -> u64 {
    let mut visited = HashSet::new();
    let (mut sx, mut sy) = (0_isize, 0_isize);
    let (mut rx, mut ry) = (0_isize, 0_isize);
    for chunk in input.chunks(2) {
        let &[(dx1, dy1), (dx2, dy2)] = chunk else {
            panic!()
        };

        visited.insert((sx, sy));
        sx += dx1;
        sy += dy1;

        visited.insert((rx, ry));
        rx += dx2;
        ry += dy2;
    }
    visited.insert((sx, sy));
    visited.insert((rx, ry));

    visited.len() as _
}

crate::solution!(
    Input,
    3,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
