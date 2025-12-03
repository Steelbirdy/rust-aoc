use crate::{Sample, samples};

type Input = Vec<Reindeer>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

const TOTAL_TIME: u64 = 2503;

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
#[prse = "{name} can fly {speed} km/s for {time} seconds, but then must rest for {rest_time} seconds."]
pub struct Reindeer {
    name: String,
    speed: u64,
    time: u64,
    rest_time: u64,
}

impl Reindeer {
    fn cycle_time(&self) -> u64 {
        self.time + self.rest_time
    }

    fn cycle_distance(&self) -> u64 {
        self.time * self.speed
    }

    fn partial_distance(&self, seconds: u64) -> u64 {
        assert!(seconds <= self.cycle_time());
        if seconds <= self.time {
            seconds * self.speed
        } else {
            self.cycle_distance()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ReindeerState<'a> {
    reindeer: &'a Reindeer,
    current_time: u64,
    current_distance: u64,
    points: u64,
}

impl ReindeerState<'_> {
    fn increment_time(&mut self) {
        if self.current_time == self.reindeer.cycle_time() {
            self.current_time = 0;
        }
        if self.current_time < self.reindeer.time {
            self.current_distance += self.reindeer.speed;
        }
        self.current_time += 1;
    }
}

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| prse::parse!(line, "{}")).collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut highest_distance = 0;
    for reindeer in input {
        let complete_cycles = TOTAL_TIME / reindeer.cycle_time();
        let partial_seconds = TOTAL_TIME % reindeer.cycle_time();
        let distance = reindeer.cycle_distance() * complete_cycles
            + reindeer.partial_distance(partial_seconds);
        highest_distance = highest_distance.max(distance);
    }

    highest_distance
}

pub fn part2(input: &Input) -> u64 {
    let mut states: Vec<_> = input
        .iter()
        .map(|reindeer| ReindeerState {
            reindeer,
            current_time: 0,
            current_distance: 0,
            points: 0,
        })
        .collect();

    for _ in 0..TOTAL_TIME {
        let mut highest_index = Vec::new();
        let mut highest_distance = 0;
        for (i, reindeer) in states.iter_mut().enumerate() {
            reindeer.increment_time();
            if reindeer.current_distance == highest_distance {
                highest_index.push(i);
            } else if reindeer.current_distance > highest_distance {
                highest_index.clear();
                highest_index.push(i);
                highest_distance = reindeer.current_distance;
            }
        }
        for i in highest_index.drain(..) {
            states[i].points += 1;
        }
    }

    states
        .iter()
        .max_by_key(|state| state.points)
        .unwrap()
        .points
}

crate::solution!(
    Input,
    2015,
    14,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
