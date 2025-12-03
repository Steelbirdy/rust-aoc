use crate::{samples, Sample};

type Input = Vec<(u32, u32, u32)>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn is_valid_triangle(a: u32, b: u32, c: u32) -> bool {
    let mut arr = [a, b, c];
    arr.sort();
    let [a, b, c] = arr;
    a + b > c
}

pub fn generator(input: &str) -> Input {
    let pattern = regex::Regex::new(r"(\d+)[\s\t]+(\d+)[\s\t]+(\d+)").unwrap();
    input.lines()
        .map(|line| {
            let captures = pattern.captures(line.trim()).unwrap();
            (captures[1].parse().unwrap(), captures[2].parse().unwrap(), captures[3].parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input.iter()
        .filter(|&&(a, b, c)| is_valid_triangle(a, b, c))
        .count() as u64
}

pub fn part2(input: &Input) -> u64 {
    input.chunks_exact(3)
        .flat_map(|chunk| {
            let &[(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)] = chunk
                else { panic!() };
            [(a1, b1, c1), (a2, b2, c2), (a3, b3, c3)]
        })
        .filter(|&(a, b, c)| is_valid_triangle(a, b, c))
        .count() as u64
}

crate::solution!(Input, 2016, 3, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
