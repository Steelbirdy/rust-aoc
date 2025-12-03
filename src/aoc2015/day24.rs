use crate::{Sample, samples};
use itertools::Itertools;

type Input = Vec<u32>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &Input) -> u64 {
    let total_weight = input.iter().copied().sum::<u32>();
    assert_eq!(total_weight % 3, 0);

    let weight_of_group = total_weight / 3;

    let mut minimal_len = usize::MAX;
    let mut minimal_qe = u64::MAX;
    for group_len in 2..input.len() / 3 {
        if group_len > minimal_len {
            break;
        }

        for combination in input.iter().copied().combinations(group_len) {
            if combination.iter().copied().sum::<u32>() != weight_of_group {
                continue;
            }
            let qe = combination.iter().copied().map(u64::from).product::<u64>();
            if qe > minimal_qe {
                continue;
            }
            let mut rest = input.clone();
            rest.retain(|x| !combination.contains(x));

            'second: for second_group_len in 2..(rest.len() - group_len) {
                for second_combination in rest.iter().copied().combinations(second_group_len) {
                    if second_combination.iter().copied().sum::<u32>() != weight_of_group {
                        continue;
                    }
                    minimal_len = group_len;
                    minimal_qe = qe;
                    break 'second;
                }
            }
        }
    }

    minimal_qe
}

pub fn part2(input: &Input) -> u64 {
    let total_weight = input.iter().copied().sum::<u32>();
    assert_eq!(total_weight % 4, 0);

    let weight_of_group = total_weight / 4;

    let mut minimal_len = usize::MAX;
    let mut minimal_qe = u64::MAX;
    for group_len in 2..input.len() / 4 {
        if group_len > minimal_len {
            break;
        }

        for combination in input.iter().copied().combinations(group_len) {
            if combination.iter().copied().sum::<u32>() != weight_of_group {
                continue;
            }
            let qe = combination.iter().copied().map(u64::from).product::<u64>();
            if qe > minimal_qe {
                continue;
            }
            let mut rest = input.clone();
            rest.retain(|x| !combination.contains(x));

            'second: for second_group_len in 2..(rest.len() - group_len) {
                for second_combination in rest.iter().copied().combinations(second_group_len) {
                    if second_combination.iter().copied().sum::<u32>() != weight_of_group {
                        continue;
                    }
                    let mut rest = rest.clone();
                    rest.retain(|x| !second_combination.contains(x));

                    for third_group_len in 2..(rest.len() - group_len - second_group_len) {
                        for third_combination in rest.iter().copied().combinations(third_group_len)
                        {
                            if third_combination.iter().copied().sum::<u32>() != weight_of_group {
                                continue;
                            }
                            minimal_len = group_len;
                            minimal_qe = qe;
                            break 'second;
                        }
                    }
                }
            }
        }
    }

    minimal_qe
}

crate::solution!(
    Input,
    2015,
    24,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
