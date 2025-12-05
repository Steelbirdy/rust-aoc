use crate::{Sample, samples, utils::Direction};
use std::collections::HashMap;

type Input = Vec<Vec<Direction>>;

const PART1_CHECKS: &[Sample] = samples![
    "ULL\nRRDDD\nLURDL\nUUUUD" => 1985,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|d| Direction::from_char(d).unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    let mut x = 1;
    let mut y = 1;

    let mut buttons = 0;
    for line in input {
        for &d in line {
            let (dx, dy) = d.deltas();
            x = (x + dx).clamp(0, 2);
            y = (y + dy).clamp(0, 2);
        }
        buttons = buttons * 10 + 3 * y + x + 1;
    }

    format!("{buttons}")
}

pub fn part2(input: &Input) -> String {
    let button_map = HashMap::from([
        ((0, 0), "1"),
        ((-1, 1), "2"),
        ((0, 1), "3"),
        ((1, 1), "4"),
        ((-2, 2), "5"),
        ((-1, 2), "6"),
        ((0, 2), "7"),
        ((1, 2), "8"),
        ((2, 2), "9"),
        ((-1, 3), "A"),
        ((0, 3), "B"),
        ((1, 3), "C"),
        ((0, 4), "D"),
    ]);

    let mut x = -2;
    let mut y = 2;

    let mut buttons = String::new();
    for line in input {
        for &d in line {
            let (dx, dy) = d.deltas();
            if !button_map.contains_key(&(x + dx, y + dy)) {
                continue;
            }
            x = x + dx;
            y = y + dy;
        }
        buttons.push_str(button_map[&(x, y)]);
    }

    buttons
}

crate::solution!(
    Input,
    2016,
    2,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
