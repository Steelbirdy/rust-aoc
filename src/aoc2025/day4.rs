use crate::{Sample, samples};
use rayon::prelude::*;

type Input = Vec<Vec<bool>>;

const PART1_CHECKS: &[Sample] = samples![
r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."# => 13,
];

const PART2_CHECKS: &[Sample] = samples![
r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."# => 43,
];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let (width, height) = (input[0].len(), input.len());

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .copied()
                .enumerate()
                .map(move |(x, c)| (x, y, c))
        })
        .filter(|&(x, y, c)| {
            c && crate::utils::neighbors((x, y), (width, height))
                .filter(|&(x, y)| input[y][x])
                .count()
                < 4
        })
        .count() as u64
}

pub fn part2(input: &Input) -> u64 {
    let mut grid = input.to_vec();
    let (width, height) = (input[0].len(), input.len());

    let mut removed = 0;
    let mut last_removed;
    loop {
        last_removed = removed;

        removed += (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                if !grid[y][x] {
                    return false;
                }
                let active_neighbors = crate::utils::neighbors((x, y), (width, height))
                    .filter(|&(x, y)| grid[y][x])
                    .count();

                let removed = active_neighbors < 4;
                grid[y][x] = !removed;
                removed
            })
            .count();

        if removed == last_removed {
            break;
        }
    }

    removed as u64
}

crate::solution!(
    Input,
    2025,
    4,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
