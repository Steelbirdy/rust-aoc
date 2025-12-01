use crate::{Sample, samples};

type Input = Vec<Ingredient>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
#[prse = "{name}: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}"]
pub struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn scores(&self, mult: i64) -> (i64, i64, i64, i64, i64) {
        (
            self.capacity * mult,
            self.durability * mult,
            self.flavor * mult,
            self.texture * mult,
            self.calories * mult,
        )
    }
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| prse::parse!(line.trim(), "{}"))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    assert_eq!(input.len(), 4);
    let [i1, i2, i3, i4] = input.as_slice() else {
        unreachable!()
    };

    let mut max_score = 0;
    for n1 in 0..=100 {
        let (c1, d1, f1, t1, _) = i1.scores(n1);
        for n2 in 0..=(100 - n1) {
            let (c2, d2, f2, t2, _) = i2.scores(n2);
            for n3 in 0..=(100 - n1 - n2) {
                let (c3, d3, f3, t3, _) = i3.scores(n3);
                let n4 = 100 - n1 - n2 - n3;
                let (c4, d4, f4, t4, _) = i4.scores(n4);
                let (c, d, f, t) = (
                    c1 + c2 + c3 + c4,
                    d1 + d2 + d3 + d4,
                    f1 + f2 + f3 + f4,
                    t1 + t2 + t3 + t4,
                );
                let score = c.max(0) * d.max(0) * f.max(0) * t.max(0);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }

    max_score as u64
}

pub fn part2(input: &Input) -> u64 {
    assert_eq!(input.len(), 4);
    let [i1, i2, i3, i4] = input.as_slice() else {
        unreachable!()
    };

    let mut max_score = 0;
    for n1 in 0..=100 {
        let (c1, d1, f1, t1, cal1) = i1.scores(n1);
        for n2 in 0..=(100 - n1) {
            let (c2, d2, f2, t2, cal2) = i2.scores(n2);
            for n3 in 0..=(100 - n1 - n2) {
                let (c3, d3, f3, t3, cal3) = i3.scores(n3);
                let n4 = 100 - n1 - n2 - n3;
                let (c4, d4, f4, t4, cal4) = i4.scores(n4);
                if cal1 + cal2 + cal3 + cal4 != 500 {
                    continue;
                }
                let (c, d, f, t) = (
                    c1 + c2 + c3 + c4,
                    d1 + d2 + d3 + d4,
                    f1 + f2 + f3 + f4,
                    t1 + t2 + t3 + t4,
                );
                let score = c.max(0) * d.max(0) * f.max(0) * t.max(0);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }

    max_score as u64
}

crate::solution!(
    Input,
    15,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
