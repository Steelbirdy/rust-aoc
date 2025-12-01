use crate::{Sample, samples};

type Input = Vec<String>;

const PART1_CHECKS: &[Sample] = samples![
    r#""""# => 2,
    r#""abc""# => 2,
    r#""aaa\"aaa""# => 3,
    r#""\x27""# => 5,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.lines().map(str::to_string).collect()
}

pub fn part1(input: &Input) -> u64 {
    let pattern = regex::Regex::new(r#"\\\\|\\"|\\x[0-9a-f]{2}"#).unwrap();
    let mut res = 0_i64;
    for line in input {
        let len = line.len() as u64;
        let code = len;
        let mut values = len - 2;
        for mt in pattern.find_iter(line) {
            values -= if mt.as_str().contains('x') { 3 } else { 1 }
        }

        res += code as i64 - values as i64;
    }

    res as u64
}

pub fn part2(input: &Input) -> u64 {
    let pattern = regex::Regex::new(r#"["\\]"#).unwrap();
    let mut res = 0_i64;
    for line in input {
        let code = line.len() as u64;
        let mut values = (line.len() + 4) as u64;
        values += pattern.find_iter(&line[1..line.len() - 1]).count() as u64;
        res += values as i64 - code as i64;
    }

    res as u64
}

crate::solution!(
    Input,
    8,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
