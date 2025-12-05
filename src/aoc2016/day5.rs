use crate::{Sample, samples};

type Input = String;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> String {
    let mut password = String::new();
    for i in 0..u32::MAX {
        let id_and_index = format!("{input}{i}");

        let bytes = md5::compute(id_and_index).0;
        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] & 0xF0 == 0 {
            let ch = bytes[2] & 0x0F;
            password = format!("{password}{ch:x}");
            println!("Found {ch:x} @ i={i}");
        }
        if password.len() == 8 {
            break;
        }
    }
    password
}

pub fn part2(input: &Input) -> String {
    let mut password = vec![u8::MAX; 8];
    for i in 0..u32::MAX {
        let id_and_index = format!("{input}{i}");

        let bytes = md5::compute(id_and_index).0;
        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] & 0xF0 == 0 {
            let index = usize::from(bytes[2] & 0x0F);
            if index > 7 || password[index] != u8::MAX {
                continue;
            }
            let ch = (bytes[3] & 0xF0) >> 4;
            password[index] = ch;
            println!("Found {ch:x} for index {index} @ i={i}");
        }
        if password.iter().all(|&ch| ch != u8::MAX) {
            break;
        }
    }
    password.into_iter().map(|b| format!("{b:x}")).collect()
}

crate::solution!(
    Input,
    2016,
    5,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
