use crate::{Sample, samples};
use prse::{Parse, parse};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Parse)]
pub enum Instruction {
    #[prse = "turn on {}"]
    TurnOn(Command),
    #[prse = "turn off {}"]
    TurnOff(Command),
    #[prse = "toggle {}"]
    Toggle(Command),
}

impl Instruction {
    pub fn command(&self) -> &Command {
        match self {
            Self::TurnOn(c) => c,
            Self::TurnOff(c) => c,
            Self::Toggle(c) => c,
        }
    }

    pub fn execute_bool(&self) -> fn(bool) -> bool {
        match self {
            Self::TurnOn(_) => |_| true,
            Self::TurnOff(_) => |_| false,
            Self::Toggle(_) => |b| !b,
        }
    }

    pub fn execute_u64(&self) -> fn(u64) -> u64 {
        match self {
            Self::TurnOn(_) => |b| b + 1,
            Self::TurnOff(_) => |b| b.saturating_sub(1),
            Self::Toggle(_) => |b| b + 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Parse)]
#[prse = "{start_x},{start_y} through {end_x},{end_y}"]
pub struct Command {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}

type Input = Vec<Instruction>;

const PART1_CHECKS: &[Sample] = samples![
    "turn on 0,0 through 999,999" => 1_000_000,
    "toggle 0,0 through 999,0" => 1_000,
    "turn off 499,499 through 500,500" => 0,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| parse!(line, "{}")).collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut arr = vec![vec![false; 1_000]; 1_000];
    for instruction in input {
        let f = instruction.execute_bool();
        let command = instruction.command();
        for y in &mut arr[command.start_y..=command.end_y] {
            for x in &mut y[command.start_x..=command.end_x] {
                *x = f(*x);
            }
        }
    }

    arr.iter().flatten().filter(|&&b| b).count() as _
}

pub fn part2(input: &Input) -> u64 {
    let mut arr = vec![vec![0; 1_000]; 1_000];
    for instruction in input {
        let f = instruction.execute_u64();
        let command = instruction.command();
        for y in &mut arr[command.start_y..=command.end_y] {
            for x in &mut y[command.start_x..=command.end_x] {
                *x = f(*x);
            }
        }
    }

    arr.iter().flatten().sum()
}

crate::solution!(
    Input,
    6,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
