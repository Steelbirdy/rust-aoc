use crate::{Sample, samples, utils};
use std::collections::HashMap;

type Input = Vec<(String, GainOrLose, i64, String)>;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum GainOrLose {
    #[prse = "gain"]
    Gain,
    #[prse = "lose"]
    Lose,
}

fn build_happiness_map(input: &Input) -> HashMap<(&str, &str), i64> {
    let mut happiness_map: HashMap<_, i64> = HashMap::new();
    for (p1, gl, n, p2) in input {
        let (mut p1, mut p2) = (p1, p2);
        if p1 > p2 {
            std::mem::swap(&mut p1, &mut p2);
        }
        let n = if *gl == GainOrLose::Gain { *n } else { -*n };
        let happiness_score = happiness_map.entry((p1.as_str(), p2.as_str())).or_default();
        *happiness_score += n;
    }

    happiness_map
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            prse::parse!(
                line,
                "{} would {} {} happiness units by sitting next to {}."
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let happiness_map = build_happiness_map(input);

    let mut graph = petgraph::Graph::new_undirected();
    let mut nodes_map = HashMap::new();
    for (&(p1, p2), &happiness) in &happiness_map {
        let n1 = *nodes_map.entry(p1).or_insert_with(|| graph.add_node(p1));
        let n2 = *nodes_map.entry(p2).or_insert_with(|| graph.add_node(p2));
        graph.add_edge(n1, n2, -happiness);
    }

    let (_, shortest_length) = utils::traveling_salesman_naive(&graph, true);
    u64::try_from(-shortest_length).unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let happiness_map = build_happiness_map(input);

    let mut graph = petgraph::Graph::new_undirected();
    let mut nodes_map = HashMap::new();
    for (&(p1, p2), &happiness) in &happiness_map {
        let n1 = *nodes_map.entry(p1).or_insert_with(|| graph.add_node(p1));
        let n2 = *nodes_map.entry(p2).or_insert_with(|| graph.add_node(p2));
        graph.add_edge(n1, n2, -happiness);
    }

    let you_node = graph.add_node("you");
    for &node_index in nodes_map.values() {
        graph.add_edge(you_node, node_index, 0);
    }

    let (_, shortest_length) = utils::traveling_salesman_naive(&graph, true);
    u64::try_from(-shortest_length).unwrap()
}

crate::solution!(
    Input,
    2015,
    13,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
