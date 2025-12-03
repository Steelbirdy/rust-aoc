use crate::{Sample, samples};
use serde_json::Value;

type Input = String;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> u64 {
    let blob = serde_json::from_str::<serde_json::Value>(input).unwrap();
    let mut values = vec![blob];
    let mut sum = 0_i64;
    while let Some(value) = values.pop() {
        match value {
            Value::Null | Value::Bool(_) | Value::String(_) => {}
            Value::Number(x) => {
                sum += x.as_i64().unwrap();
            }
            Value::Array(arr) => {
                values.extend(arr);
            }
            Value::Object(obj) => {
                values.extend(obj.into_values());
            }
        }
    }

    sum as u64
}

pub fn part2(input: &Input) -> u64 {
    let blob = serde_json::from_str::<serde_json::Value>(input).unwrap();
    let mut values = vec![blob];
    let mut sum = 0_i64;
    while let Some(value) = values.pop() {
        match value {
            Value::Null | Value::Bool(_) | Value::String(_) => {}
            Value::Number(x) => {
                sum += x.as_i64().unwrap();
            }
            Value::Array(arr) => {
                values.extend(arr);
            }
            Value::Object(obj) => {
                if obj.values().any(|x| x.as_str() == Some("red")) {
                    continue;
                }
                values.extend(obj.into_values());
            }
        }
    }

    sum as u64
}

crate::solution!(
    Input,
    2015,
    12,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
