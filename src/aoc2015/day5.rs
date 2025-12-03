use crate::{Sample, samples};
use std::collections::HashSet;

type Input = Vec<String>;

pub const PART1_CHECKS: &[Sample] = samples![
    "ugknbfddgicrmopn" => 1,
    "aaa" => 1,
    "jchzalrnumimnmhp" => 0,
    "haegwjzuvuyypxyu" => 0,
    "dvszwmarrgswjxmb" => 0,
];

pub const PART2_CHECKS: &[Sample] = samples![
    "qjhvhtzxzqqjkmpb" => 1,
    "xxyxx" => 1,
    "uurcxstgmygtbstg" => 0,
    "ieodomkazucvgmuy" => 0,
];

pub fn generator(input: &str) -> Input {
    input.lines().map(str::to_string).collect()
}

pub fn part1(input: &Input) -> u64 {
    fn is_nice(s: &str) -> bool {
        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        const BAD_PAIRS: [(u8, u8); 4] = [(b'a', b'b'), (b'c', b'd'), (b'p', b'q'), (b'x', b'y')];

        let bytes = s.as_bytes();

        let mut vowels = 0;
        let mut double = false;
        for b in bytes.windows(2) {
            let &[b1, b2] = b else { panic!() };

            if BAD_PAIRS.contains(&(b1, b2)) {
                return false;
            }

            if b1 == b2 {
                double = true;
            }
            if VOWELS.contains(&b2) {
                vowels += 1;
            }
        }

        if VOWELS.contains(&bytes[0]) {
            vowels += 1;
        }

        vowels >= 3 && double
    }

    input.iter().filter(|s| is_nice(s)).count() as _
}

pub fn part2(input: &Input) -> u64 {
    fn is_nice(s: &str) -> bool {
        let bytes = s.as_bytes();

        let mut pairs = HashSet::new();

        let mut repeating_pair = false;
        let mut repeating_letter = false;

        for window in bytes.windows(3) {
            let &[p, a, b] = window else { panic!() };

            if p == b {
                repeating_letter = true;
            }

            if pairs.contains(&(a, b)) {
                repeating_pair = true;
            }

            pairs.insert((p, a));
        }

        repeating_pair && repeating_letter
    }

    input.iter().filter(|s| is_nice(s)).count() as _
}

crate::solution!(
    Input,
    2015,
    5,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
