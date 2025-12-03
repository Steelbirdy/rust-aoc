use crate::{Sample, samples};
use rayon::prelude::*;

type Input = u64;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: &Input) -> u64 {
    let desired = *input / 10;

    (1..=desired)
        .into_par_iter()
        .map(|n| {
            let factorization = prime_factorization::Factorization::run(n);
            let sum_of_factors = factorization
                .prime_factor_repr()
                .into_iter()
                .map(|(pi, mi)| (pi.pow(mi + 1) - 1) / (pi - 1))
                .product::<u64>();
            (n, sum_of_factors)
        })
        .find_map_first(|(n, p)| (p >= desired).then_some(n))
        .unwrap()
}

pub fn part2(input: &Input) -> u64 {
    // Only includes divisors N/50 <= d <= N
    let desired = input.div_ceil(11);

    fn calculate_without_small_divisors(n: u64) -> u64 {
        // only check divisibility between n and numbers <= 50
        (1..=50_u64)
            .filter_map(|d| n.is_multiple_of(d).then_some(n / d))
            .sum()
    }

    (1..=*input / 10)
        .into_par_iter()
        .find_first(|&n| calculate_without_small_divisors(n) > desired)
        .unwrap()
}

crate::solution!(
    Input,
    2015,
    20,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
