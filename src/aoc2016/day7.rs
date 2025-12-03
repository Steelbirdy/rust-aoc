use itertools::Itertools;
use crate::{samples, Sample};

type Input = Vec<String>;

const PART1_CHECKS: &[Sample] = samples![
    "abba[mnop]qrst" => 1,
    "abcd[bddb]xyyx" => 0,
    "aaaa[qwer]tyui" => 0,
    "ioxxoj[asdfgh]zxcvbn" => 1,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.lines().map(String::from).collect()
}

pub fn part1(input: &Input) -> u64 {
    fn has_abba(s: &str) -> bool {
        s.as_bytes()
            .windows(4)
            .any(|window| window[0] != window[1] && window[0] == window[3] && window[1] == window[2])
    }

    let pattern = regex::Regex::new(r"\[([a-z]+)]").unwrap();
    input.iter()
        .filter(|line| {
            let has_abba_outside_brackets = pattern.split(line)
                .any(has_abba);
            let has_abba_in_brackets = pattern.find_iter(line)
                .any(|m| has_abba(m.as_str()));
            has_abba_outside_brackets && !has_abba_in_brackets
        })
        .count() as u64
}

pub fn part2(input: &Input) -> u64 {
    fn abas(s: &str) -> impl Iterator<Item = (u8, u8)> {
        s.as_bytes()
            .windows(3)
            .filter_map(|window| {
                let is_aba = window[0] != window[1] && window[0] == window[2];
                is_aba.then_some((window[0], window[1]))
            })
    }

    let pattern = regex::Regex::new(r"\[([a-z]+)]").unwrap();
    input.iter()
        .filter(|line| {
            let abas_outside_brackets = pattern.split(line)
                .flat_map(abas);
            let abas_inside_brackets: Vec<_> = pattern.find_iter(line)
                .flat_map(|m| abas(m.as_str()))
                .collect();
            abas_outside_brackets.cartesian_product(abas_inside_brackets)
                .any(|((x1, y1), (x2, y2))| x1 == y2 && y1 == x2)
        })
        .count() as u64
}

crate::solution!(Input, 2016, 7, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
