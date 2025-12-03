use crate::{Sample, samples};
use rayon::prelude::*;

type Input = Vec<Vec<bool>>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

const SIZE: usize = 100;

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    const ALL_NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    ALL_NEIGHBORS.into_iter().filter_map(move |(dx, dy)| {
        let x = x.checked_sub_signed(dx)?;
        let y = y.checked_sub_signed(dy)?;
        (x < w && y < h).then_some((x, y))
    })
}

fn update(current: &[Vec<bool>], next: &mut [Vec<bool>]) {
    next.par_iter_mut()
        .enumerate()
        .flat_map_iter(|(y, row)| row.iter_mut().enumerate().map(move |(x, b)| (x, y, b)))
        .for_each(|(x, y, b)| {
            let on_neighbors = neighbors(x, y, SIZE, SIZE)
                .filter(|&(x, y)| current[y][x])
                .count();
            *b = if current[y][x] {
                (2..=3).contains(&on_neighbors)
            } else {
                on_neighbors == 3
            }
        });
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut current = input.to_vec();
    let mut next = vec![vec![false; SIZE]; SIZE];
    for _ in 0..100 {
        update(&current, &mut next);
        std::mem::swap(&mut current, &mut next);
    }
    current.iter().flatten().filter(|&&b| b).count() as u64
}

pub fn part2(input: &Input) -> u64 {
    let mut current = input.to_vec();
    let mut next = vec![vec![false; SIZE]; SIZE];
    for _ in 0..100 {
        current[0][0] = true;
        current[0][SIZE - 1] = true;
        current[SIZE - 1][0] = true;
        current[SIZE - 1][SIZE - 1] = true;
        update(&current, &mut next);
        std::mem::swap(&mut current, &mut next);
    }

    current[0][0] = true;
    current[0][SIZE - 1] = true;
    current[SIZE - 1][0] = true;
    current[SIZE - 1][SIZE - 1] = true;
    current.iter().flatten().filter(|&&b| b).count() as u64
}

crate::solution!(
    Input,
    2015,
    18,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
