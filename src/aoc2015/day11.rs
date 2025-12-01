use crate::{Sample, samples};
use std::collections::HashSet;

type Input = Vec<u8>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

const NOT_ALLOWED: &[u8] = &[b'i' - b'a', b'o' - b'a', b'l' - b'a'];

fn check_password(input: &[u8]) -> bool {
    let mut increasing_straight = false;
    let mut pairs = HashSet::new();

    for window in input.windows(3) {
        let &[a, b, c] = window else { panic!() };

        if a + 1 == b && b + 1 == c {
            increasing_straight = true;
        }

        if a == b || b == c {
            pairs.insert(b);
        }
    }

    increasing_straight && pairs.len() >= 2
}

fn increment_password(input: &mut [u8]) {
    crate::utils::next_lexicographic(input, |b| {
        if *b == 25 {
            *b = 0;
            return true;
        }

        *b += 1;
        if NOT_ALLOWED.contains(b) {
            *b += 1;
        }
        false
    });
}

fn password_to_string(password: &[u8]) -> String {
    password.iter().map(|&b| char::from(b + b'a')).collect()
}

pub fn generator(input: &str) -> Input {
    input.bytes().map(|b| b - b'a').collect()
}

pub fn part1(input: &Input) -> String {
    let mut password = input.to_vec();
    increment_password(&mut password);
    while !check_password(&password) {
        increment_password(&mut password);
    }
    password_to_string(&password)
}

pub fn part2(input: &Input) -> String {
    let mut password = input.to_vec();
    increment_password(&mut password);
    while !check_password(&password) {
        increment_password(&mut password);
    }
    increment_password(&mut password);
    while !check_password(&password) {
        increment_password(&mut password);
    }
    password_to_string(&password)
}

crate::solution!(
    Input,
    11,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
