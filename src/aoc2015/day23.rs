use crate::{Sample, samples};

type Input = Vec<Instruction>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Instruction {
    #[prse = "hlf {}"]
    Hlf(Register),
    #[prse = "tpl {}"]
    Tpl(Register),
    #[prse = "inc {}"]
    Inc(Register),
    #[prse = "jmp {}"]
    Jmp(i16),
    #[prse = "jie {}, {}"]
    Jie(Register, i16),
    #[prse = "jio {}, {}"]
    Jio(Register, i16),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Register {
    #[prse = "a"]
    A,
    #[prse = "b"]
    B,
}

impl Register {
    fn get(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
        }
    }
}

impl Instruction {
    pub fn run(self, ip: &mut usize, registers: &mut [u64; 2]) -> Option<()> {
        match self {
            Instruction::Hlf(r) => {
                registers[r.get()] /= 2;
                *ip += 1;
            }
            Instruction::Tpl(r) => {
                registers[r.get()] *= 3;
                *ip += 1;
            }
            Instruction::Inc(r) => {
                registers[r.get()] += 1;
                *ip += 1;
            }
            Instruction::Jmp(offset) => {
                *ip = ip.checked_add_signed(offset as isize)?;
            }
            Instruction::Jie(r, offset) => {
                let value = registers[r.get()];
                if value.is_multiple_of(2) {
                    *ip = ip.checked_add_signed(offset as isize)?;
                } else {
                    *ip += 1;
                }
            }
            Instruction::Jio(r, offset) => {
                let value = registers[r.get()];
                if value == 1 {
                    *ip = ip.checked_add_signed(offset as isize)?;
                } else {
                    *ip += 1;
                }
            }
        }
        Some(())
    }
}

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| prse::parse!(line, "{}")).collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut ip = 0_usize;
    let mut registers = [0; 2];
    while ip < input.len() {
        if input[ip].run(&mut ip, &mut registers).is_none() {
            break;
        }
    }
    registers[1]
}

pub fn part2(input: &Input) -> u64 {
    let mut ip = 0_usize;
    let mut registers = [1, 0];
    while ip < input.len() {
        if input[ip].run(&mut ip, &mut registers).is_none() {
            break;
        }
    }
    registers[1]
}

crate::solution!(
    Input,
    23,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
