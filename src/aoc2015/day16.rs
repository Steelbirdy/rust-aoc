use crate::{Sample, samples};
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Input = Vec<Sue>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sue {
    n: u64,
    gifts: HashMap<String, u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
#[prse = "{kind}: {count}"]
pub struct Gift {
    kind: String,
    count: u64,
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (n, gifts): (u64, Vec<Gift>) = prse::parse!(line, "Sue {}: {:, :}");
            Sue {
                n,
                gifts: gifts.into_iter().map(|g| (g.kind, g.count)).collect(),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let expected: HashMap<_, u64> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .map(|(kind, count)| (kind.to_string(), count))
    .collect();

    input
        .iter()
        .find(|sue| {
            sue.gifts
                .iter()
                .all(|(kind, count)| expected.get(kind) == Some(count))
        })
        .unwrap()
        .n
}

pub fn part2(input: &Input) -> u64 {
    let expected: HashMap<_, RangeInclusive<u64>> = [
        ("children", 3..=3),
        ("cats", 8..=u64::MAX),
        ("samoyeds", 2..=2),
        ("pomeranians", 0..=2),
        ("akitas", 0..=0),
        ("vizslas", 0..=0),
        ("goldfish", 0..=4),
        ("trees", 4..=u64::MAX),
        ("cars", 2..=2),
        ("perfumes", 1..=1),
    ]
    .into_iter()
    .map(|(kind, count)| (kind.to_string(), count))
    .collect();

    input
        .iter()
        .find(|sue| {
            sue.gifts.iter().all(|(kind, count)| {
                expected
                    .get(kind)
                    .is_some_and(|range| range.contains(count))
            })
        })
        .unwrap()
        .n
}

crate::solution!(
    Input,
    2015,
    16,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
