use std::collections::HashMap;
use crate::{samples, Sample};

type Input = Vec<Instruction>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Instruction {
    #[prse = "value {value} goes to bot {goes_to_bot}"]
    GoesTo {
        value: u32,
        goes_to_bot: u32,
    },
    #[prse = "bot {bot} gives low to {low_output_or_bot} {low} and high to {high_output_or_bot} {high}"]
    Gives {
        bot: u32,
        low_output_or_bot: OutputOrBot,
        low: u32,
        high_output_or_bot: OutputOrBot,
        high: u32,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum OutputOrBot {
    #[prse = "output"]
    Output,
    #[prse = "bot"]
    Bot,
}

pub fn generator(input: &str) -> Input {
    input.lines()
        .map(|line| prse::parse!(line.trim(), "{}"))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut bots: HashMap<u32, Vec<u32>> = HashMap::new();

    let (value_instr, mut gives_instr): (Vec<_>, Vec<_>) = input.iter()
        .copied()
        .partition(|instr| matches!(instr, Instruction::GoesTo { .. }));

    for instruction in value_instr {
        let Instruction::GoesTo { value, goes_to_bot } = instruction
        else { unreachable!() };
        let chips = bots
            .entry(goes_to_bot)
            .or_default();
        chips.push(value);
        chips.sort();
    }

    let mut found_it: Option<u32> = None;
    while !gives_instr.is_empty() {
        gives_instr.retain(|&instr| {
            let Instruction::Gives { bot, low_output_or_bot, low, high_output_or_bot, high } = instr
                else { unreachable!() };

            let bot_len = bots.entry(bot)
                .or_default()
                .len();

            if bot_len < 2 {
                return true;
            }

            let chips = std::mem::take(bots.get_mut(&bot).unwrap());
            let (low_chip, high_chip) = (chips[0], chips[1]);

            if low_chip == 17 && high_chip == 61 {
                found_it = Some(bot);
            }

            if low_output_or_bot == OutputOrBot::Bot {
                let low_chips = bots.entry(low)
                    .or_default();
                low_chips.push(low_chip);
                low_chips.sort();
            }

            if high_output_or_bot == OutputOrBot::Bot {
                let high_chips = bots.entry(high)
                    .or_default();
                high_chips.push(high_chip);
                high_chips.sort();
            }

            false
        });
        if let Some(bot) = found_it.take() {
            return bot as u64
        }
    }

    panic!("not found")
}

pub fn part2(input: &Input) -> u64 {
    let mut bots: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut outputs: HashMap<u32, u32> = HashMap::new();

    let (value_instr, mut gives_instr): (Vec<_>, Vec<_>) = input.iter()
        .copied()
        .partition(|instr| matches!(instr, Instruction::GoesTo { .. }));

    for instruction in value_instr {
        let Instruction::GoesTo { value, goes_to_bot } = instruction
        else { unreachable!() };
        let chips = bots
            .entry(goes_to_bot)
            .or_default();
        chips.push(value);
        chips.sort();
    }

    while !gives_instr.is_empty() {
        gives_instr.retain(|&instr| {
            let Instruction::Gives { bot, low_output_or_bot, low, high_output_or_bot, high } = instr
            else { unreachable!() };

            let bot_len = bots.entry(bot)
                .or_default()
                .len();

            if bot_len < 2 {
                return true;
            }

            let chips = std::mem::take(bots.get_mut(&bot).unwrap());
            let (low_chip, high_chip) = (chips[0], chips[1]);

            if low_output_or_bot == OutputOrBot::Bot {
                let low_chips = bots.entry(low)
                    .or_default();
                low_chips.push(low_chip);
                low_chips.sort();
            } else {
                outputs.insert(low, low_chip);
            }

            if high_output_or_bot == OutputOrBot::Bot {
                let high_chips = bots.entry(high)
                    .or_default();
                high_chips.push(high_chip);
                high_chips.sort();
            } else {
                outputs.insert(high, high_chip);
            }

            false
        });
    }

    (outputs[&0] * outputs[&1] * outputs[&2]) as u64
}

crate::solution!(Input, 10, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
