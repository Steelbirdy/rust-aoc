use crate::{samples, Sample};

type Input = Vec<Instruction>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Instruction {
    #[prse = "rect {}x{}"]
    Rect(u32, u32),
    #[prse = "rotate column x={} by {}"]
    RotateColumn(u32, u32),
    #[prse = "rotate row y={} by {}"]
    RotateRow(u32, u32),
}

pub fn generator(input: &str) -> Input {
    input.lines()
        .map(|line| prse::parse!(line.trim(), "{}"))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    let mut screen = [[false; WIDTH]; HEIGHT];
    for instruction in input {
        match *instruction {
            Instruction::Rect(w, h) => {
                for row in &mut screen[0..h as usize] {
                    for px in &mut row[0..w as usize] {
                        *px = true;
                    }
                }
            }
            Instruction::RotateColumn(x, offset) => {
                let column: Vec<_> = screen.iter()
                    .map(|row| row[x as usize])
                    .collect();
                for y in 0..HEIGHT {
                    let y_offset = (y + offset as usize) % HEIGHT;
                    screen[y_offset][x as usize] = column[y];
                }
            }
            Instruction::RotateRow(y, offset) => {
                let row = screen[y as usize].to_vec();
                for x in 0..WIDTH {
                    let x_offset = (x + offset as usize) % WIDTH;
                    screen[y as usize][x_offset] = row[x];
                }
            }
        }
    }

    screen.into_iter()
        .flatten()
        .filter(|px| *px)
        .count() as u64
}

pub fn part2(input: &Input) -> u64 {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    let mut screen = [[false; WIDTH]; HEIGHT];
    for instruction in input {
        match *instruction {
            Instruction::Rect(w, h) => {
                for row in &mut screen[0..h as usize] {
                    for px in &mut row[0..w as usize] {
                        *px = true;
                    }
                }
            }
            Instruction::RotateColumn(x, offset) => {
                let column: Vec<_> = screen.iter()
                    .map(|row| row[x as usize])
                    .collect();
                for y in 0..HEIGHT {
                    let y_offset = (y + offset as usize) % HEIGHT;
                    screen[y_offset][x as usize] = column[y];
                }
            }
            Instruction::RotateRow(y, offset) => {
                let row = screen[y as usize].to_vec();
                for x in 0..WIDTH {
                    let x_offset = (x + offset as usize) % WIDTH;
                    screen[y as usize][x_offset] = row[x];
                }
            }
        }
    }

    screen.into_iter()
        .for_each(|row| {
            let st: String = row.into_iter()
                .map(|px| if px { '#' } else { ' ' })
                .collect();
            println!("{}", st);
        });

    0
}

crate::solution!(Input, 2016, 8, generator, part1, PART1_CHECKS, part2, PART2_CHECKS);
