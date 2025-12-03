use std::collections::HashMap;
use itertools::Itertools;
use crate::{samples, Sample};

type Input = Vec<(String, u64, String)>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

fn get_real_rooms(rooms: &[(String, u64, String)]) -> Vec<(&str, u64)> {
    let mut result = Vec::new();
    for (name, sector_id, checksum) in rooms {
        let most_common = name
            .chars()
            .counts();
        let mut most_common_inverse: HashMap<usize, Vec<char>> = HashMap::new();
        for (ch, count) in most_common {
            most_common_inverse.entry(count)
                .or_default()
                .push(ch)
        }
        for entry in most_common_inverse.values_mut() {
            entry.sort();
        }
        let mut most_common: Vec<_> = most_common_inverse
            .into_iter()
            .collect();
        most_common.sort_by_key(|&(x, _)| std::cmp::Reverse(x));

        let mut guessed_checksum = Vec::new();
        for (_, chars) in most_common {
            guessed_checksum.extend(chars);
            if guessed_checksum.len() >= 5 {
                break
            }
        }
        let guessed_checksum: String = guessed_checksum[..5]
            .iter()
            .copied()
            .collect();

        if *checksum == guessed_checksum {
            result.push((name.as_str(), *sector_id));
        }
    }

    result
}

pub fn generator(input: &str) -> Input {
    let pattern = regex::Regex::new(r"([a-z\-]+)-(\d+)\[([a-z]+)]").unwrap();
    input.lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                captures[1].replace('-', ""),
                captures[2].parse().unwrap(),
                captures[3].to_string(),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for (name, sector_id, checksum) in input {
        let most_common = name
            .chars()
            .counts();
        let mut most_common_inverse: HashMap<usize, Vec<char>> = HashMap::new();
        for (ch, count) in most_common {
            most_common_inverse.entry(count)
                .or_default()
                .push(ch)
        }
        for entry in most_common_inverse.values_mut() {
            entry.sort();
        }
        let mut most_common: Vec<_> = most_common_inverse
            .into_iter()
            .collect();
        most_common.sort_by_key(|&(x, _)| std::cmp::Reverse(x));

        let mut guessed_checksum = Vec::new();
        for (_, chars) in most_common {
            guessed_checksum.extend(chars);
            if guessed_checksum.len() >= 5 {
                break
            }
        }
        let guessed_checksum: String = guessed_checksum[..5]
            .iter()
            .copied()
            .collect();

        if *checksum == guessed_checksum {
            sum += sector_id;
        }
    }

    sum
}

pub fn part2(input: &Input) -> u64 {
    let real_rooms = get_real_rooms(input);
    for (name, sector_id) in real_rooms {
        let mod_sector_id = sector_id % 26;
        let name: String = name.bytes()
            .map(|b| match b {
                b'a'..=b'z' => {
                    char::from((b - b'a' + mod_sector_id as u8) % 26 + b'a')
                }
                b'-' => ' ',
                _ => unreachable!(),
            })
            .collect();
        if name == "northpoleobjectstorage" {
            return sector_id
        }
    }

    panic!("not found")
}

crate::solution!(Input, 2016, 4, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
