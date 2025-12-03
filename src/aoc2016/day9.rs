use crate::{samples, Sample};
use std::sync::OnceLock;
use regex::Regex;

type Input = String;

const PART1_CHECKS: &[Sample] = samples![
    "ADVENT" => 6,
    "A(1x5)BC" => 7,
    "(3x3)XYZ" => 9,
    "A(2x2)BCD(2x2)EFG" => 11,
    "(6x1)(1x3)A" => 6,
    "X(8x2)(3x3)ABCY" => 18,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> u64 {
    fn process(mut s: &str) -> usize {
        static PATTERN: OnceLock<Regex> = OnceLock::new();

        let mut result = 0;
        while !s.is_empty() {
            let pattern = PATTERN.get_or_init(|| {
                Regex::new(r"\((\d+)x(\d+)\)").unwrap()
            });
            let Some(captures) = pattern.captures(s)
                else { return result + s.len() };

            let r#match = captures.get_match();
            result += r#match.start();

            let len = captures[1].parse::<usize>().unwrap();
            let count = captures[2].parse::<usize>().unwrap();
            let text_start = r#match.end();
            result += count * len;
            s = &s[text_start + len..];
        }
        result
    }

    process(input) as u64
}

pub fn part2(input: &Input) -> u64 {
    fn process(mut s: &str) -> usize {
        static PATTERN: OnceLock<Regex> = OnceLock::new();

        let mut result = 0;
        while !s.is_empty() {
            let pattern = PATTERN.get_or_init(|| {
                Regex::new(r"\((\d+)x(\d+)\)").unwrap()
            });
            let Some(captures) = pattern.captures(s)
            else { return result + s.len() };

            let r#match = captures.get_match();
            result += r#match.start();

            let len = captures[1].parse::<usize>().unwrap();
            let count = captures[2].parse::<usize>().unwrap();
            let text_start = r#match.end();
            result += count * process(&s[text_start..text_start + len]);
            s = &s[text_start + len..];
        }
        result
    }

    process(input) as u64
}

crate::solution!(Input, 2016, 9, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
