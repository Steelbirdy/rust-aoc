use std::borrow::Cow;
use crate::{samples, Sample};

type Input = String;

const PART1_CHECKS: &[Sample] = samples![
    r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "# => 4277556,
];

const PART2_CHECKS: &[Sample] = samples!["123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  " => 3263827,
];

crate::regex!(GAPS = r"\s+");

pub fn generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> u64 {
    let mut problems = Vec::new();
    for line in input.lines() {
        for (i, p) in GAPS.split(line.trim()).enumerate() {
            if p.trim().is_empty() {
                continue;
            }
            if problems.len() <= i {
                problems.push((Vec::new(), 'x'));
            }
            match p.parse::<u64>() {
                Ok(n) => problems[i].0.push(n),
                Err(_) => problems[i].1 = p.chars().next().unwrap(),
            }
        }
    }

    problems.into_iter()
        .map(|(nums, op)| match op {
            '+' => nums.into_iter().sum::<u64>(),
            '*' => nums.into_iter().product::<u64>(),
            _ => panic!("Invalid operator: `{}`", op),
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    let max_len = input.lines()
        .map(|line| line.len())
        .max().unwrap();

    let (nums, ops) = input.rsplit_once('\n').unwrap();
    let ops = pad_line(ops, max_len);
    let ops = ops.as_ref();
    let mut columns: Vec<_> = GAPS.find_iter(ops)
        .map(|m| (ops.as_bytes()[m.start() - 1], (m.start() - 1..m.end())))
        .collect();
    columns.reverse();

    let nums: Vec<_> = nums.lines().map(|line| {
        let line = pad_line(line, max_len);
        line.into_owned().into_bytes()
    }).collect();

    columns
        .into_iter()
        .map(|(op, range)| {
            let iter = range
                .rev()
                .filter_map(|i| {
                    let (_, total) = nums.iter()
                        .fold((0, None), |(trailing_zeros, index_total), line| {
                            let ch = line[i];
                            if ch == b' ' {
                                (trailing_zeros + 1, index_total)
                            } else {
                                let index_total: u64 = index_total.unwrap_or_default() * 10_u64.pow(trailing_zeros + 1) + (ch - b'0') as u64;
                                (0, Some(index_total))
                            }
                        });
                    total
                });
            match op {
                b'+' => iter.sum::<u64>(),
                b'*' => iter.product::<u64>(),
                _ => panic!("Invalid operator: `{}`", op),
            }
        })
            .sum()
}

crate::solution!(Input, 2025, 6, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);

fn pad_line(line: &str, max_len: usize) -> Cow<'_, str> {
    let pad = max_len - line.len();
    if pad == 0 {
        Cow::Borrowed(line)
    } else {
        Cow::Owned(format!("{line}{}", " ".repeat(pad)))
    }
}

#[test]
fn test_pad_line() {
    let line = "hello, world!";
    assert_eq!(pad_line(line, 13), "hello, world!");
    assert_eq!(pad_line(line, 15), "hello, world!  ");
}
