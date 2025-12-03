use crate::{Sample, samples, utils};

type Input = Vec<u64>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut jars = input.to_vec();
    jars.sort_unstable();
    let mut inclusion = vec![false; jars.len()];

    let mut count = 0;
    for _ in 0..2_u64.pow(jars.len() as u32) {
        utils::next_lexicographic(inclusion.as_mut_slice(), |b| {
            *b = !*b;
            !*b
        });
        if jars
            .iter()
            .zip(&inclusion)
            .filter_map(|(&x, &b)| b.then_some(x))
            .sum::<u64>()
            == 150
        {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> u64 {
    let mut jars = input.to_vec();
    jars.sort_unstable();
    let mut inclusion = vec![false; jars.len()];

    let mut count = 0;
    let mut min_jars = jars.len();
    for _ in 0..2_u64.pow(jars.len() as u32) {
        utils::next_lexicographic(inclusion.as_mut_slice(), |b| {
            *b = !*b;
            !*b
        });
        if jars
            .iter()
            .zip(&inclusion)
            .filter_map(|(&x, &b)| b.then_some(x))
            .sum::<u64>()
            == 150
        {
            let num_jars = inclusion.iter().filter(|&&b| b).count();
            if num_jars == min_jars {
                count += 1;
            } else if num_jars < min_jars {
                count = 1;
                min_jars = num_jars;
            }
        }
    }

    count
}

crate::solution!(
    Input,
    2015,
    17,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
