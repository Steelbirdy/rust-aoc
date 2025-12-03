use crate::{Sample, samples, utils};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
#[prse = "{from} to {to} = {distance}"]
pub struct Line {
    to: String,
    from: String,
    distance: u64,
}

type Input = Vec<Line>;

const PART1_CHECKS: &[Sample] = samples![
    r"London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141" => 605
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| prse::parse!(line.trim(), "{}"))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut cities = HashMap::new();
    let mut graph = petgraph::Graph::<&str, i64, petgraph::Undirected>::default();

    for line in input {
        let to_index = *cities
            .entry(&line.to)
            .or_insert_with(|| graph.add_node(&line.to));
        let from_index = *cities
            .entry(&line.from)
            .or_insert_with(|| graph.add_node(&line.from));
        graph.add_edge(from_index, to_index, line.distance as i64);
    }

    let (_, shortest) = utils::traveling_salesman_naive(&graph, false);
    shortest as u64
}

pub fn part2(input: &Input) -> u64 {
    let mut cities = HashMap::new();
    let mut graph = petgraph::Graph::<&str, i64, petgraph::Undirected>::default();

    for line in input {
        let to_index = *cities
            .entry(&line.to)
            .or_insert_with(|| graph.add_node(&line.to));
        let from_index = *cities
            .entry(&line.from)
            .or_insert_with(|| graph.add_node(&line.from));
        graph.add_edge(from_index, to_index, -(line.distance as i64));
    }

    let (_, longest) = utils::traveling_salesman_naive(&graph, false);
    u64::try_from(-longest).unwrap()
}

crate::solution!(
    Input,
    2015,
    9,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
