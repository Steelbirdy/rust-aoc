use crate::{Sample, samples};
use prse::parse;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Instruction {
    #[prse = "NOT {}"]
    Not(Argument),
    #[prse = "{} AND {}"]
    And(Argument, Argument),
    #[prse = "{} OR {}"]
    Or(Argument, Argument),
    #[prse = "{} LSHIFT {}"]
    LShift(Argument, Argument),
    #[prse = "{} RSHIFT {}"]
    RShift(Argument, Argument),
    #[prse = "{}"]
    Assign(Argument),
}

impl Instruction {
    pub fn incoming_wires(&self) -> impl Iterator<Item = &str> {
        match self {
            Instruction::Not(a) => [Some(a), None],
            Instruction::Assign(a) => [Some(a), None],
            Instruction::And(a, b) => [Some(a), Some(b)],
            Instruction::Or(a, b) => [Some(a), Some(b)],
            Instruction::LShift(a, b) => [Some(a), Some(b)],
            Instruction::RShift(a, b) => [Some(a), Some(b)],
        }
        .into_iter()
        .filter_map(|a| match a {
            Some(Argument::Wire(w)) => Some(w.as_str()),
            _ => None,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
pub enum Argument {
    #[prse = "{}"]
    Literal(u16),
    #[prse = "{}"]
    Wire(String),
}

impl Argument {
    pub fn get(&self, wires: &HashMap<String, u16>) -> u16 {
        match self {
            Self::Literal(x) => *x,
            Self::Wire(w) => wires[w],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, prse::Parse)]
#[prse = "{instruction} -> {wire}"]
pub struct Rule {
    pub instruction: Instruction,
    pub wire: String,
}

type Input = Vec<Rule>;

const PART1_CHECKS: &[Sample] = samples![
    r"123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> a" => 65079,
];

const PART2_CHECKS: &[Sample] = samples![];

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| parse!(line.trim(), "{}"))
        .collect()
}

pub fn build_graph(rules: &[Rule]) -> petgraph::Graph<usize, (), petgraph::Directed> {
    let mut graph = petgraph::Graph::new();

    let mut rules_map: HashMap<_, _> = rules
        .iter()
        .enumerate()
        .map(|(i, rule)| (rule.wire.as_str(), (i, None)))
        .collect();

    macro_rules! insert_node {
        ($wire:expr) => {{
            let wire = $wire;
            let &(i, node_index) = rules_map
                .get(wire)
                .unwrap_or_else(|| panic!("no entry found for key {wire}"));
            let node_index = node_index.unwrap_or_else(|| graph.add_node(i));
            rules_map.insert(wire, (i, Some(node_index)));
            node_index
        }};
    }

    for rule in rules {
        let to = insert_node!(rule.wire.as_str());
        for wire in rule.instruction.incoming_wires() {
            let from = insert_node!(wire);
            graph.add_edge(from, to, ());
        }
    }

    graph
}

fn calculate(rules: &[Rule], wires: &mut HashMap<String, u16>) {
    let graph = build_graph(rules);
    let sorted_nodes = petgraph::algo::toposort(&graph, None).unwrap();

    for node_index in sorted_nodes {
        let index = graph[node_index];
        let rule = &rules[index];

        match &rule.instruction {
            Instruction::Assign(x) => {
                let x = x.get(wires);
                wires.insert(rule.wire.clone(), x);
            }
            Instruction::And(x, y) => {
                let x = x.get(wires);
                let y = y.get(wires);
                wires.insert(rule.wire.clone(), x & y);
            }
            Instruction::Or(x, y) => {
                let x = x.get(wires);
                let y = y.get(wires);
                wires.insert(rule.wire.clone(), x | y);
            }
            Instruction::Not(x) => {
                let x = x.get(wires);
                wires.insert(rule.wire.clone(), !x);
            }
            Instruction::LShift(x, y) => {
                let x = x.get(wires);
                let y = y.get(wires);
                wires.insert(rule.wire.clone(), x << y);
            }
            Instruction::RShift(x, y) => {
                let x = x.get(wires);
                let y = y.get(wires);
                wires.insert(rule.wire.clone(), x >> y);
            }
        }
    }
}

pub fn part1(input: &Input) -> u64 {
    let mut wires = HashMap::new();
    calculate(input, &mut wires);
    wires["a"] as _
}

pub fn part2(input: &Input) -> u64 {
    let mut wires = HashMap::new();
    calculate(input, &mut wires);

    let mut rules = input.to_vec();
    for rule in &mut rules {
        if rule.wire == "b" {
            rule.instruction = Instruction::Assign(Argument::Literal(wires["a"]));
            break;
        }
    }
    wires = HashMap::from([("b".to_string(), wires["a"])]);

    calculate(&rules, &mut wires);
    wires["a"] as _
}

crate::solution!(
    Input,
    7,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
