use crate::{Sample, samples};
use std::collections::{HashMap, HashSet};

pub struct Input {
    name_map: HashMap<String, u16>,
    rules: HashMap<u16, Vec<Vec<u16>>>,
    molecule: Vec<u16>,
}

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

fn split_molecule(mol: &str, name_map: &mut HashMap<String, u16>) -> Vec<u16> {
    let mut result = Vec::new();
    let mut chars = mol.chars().peekable();
    while let Some(c1) = chars.next() {
        assert!(c1.is_ascii_uppercase());
        let name = match chars.next_if(char::is_ascii_lowercase) {
            Some(c2) => format!("{c1}{c2}"),
            None => c1.to_string(),
        };
        let identifier = get_identifier(name, name_map);
        result.push(identifier);
    }

    result
}

fn get_identifier<S: AsRef<str> + ToString>(elem: S, name_map: &mut HashMap<String, u16>) -> u16 {
    name_map.get(elem.as_ref()).copied().unwrap_or_else(|| {
        let len = u16::try_from(name_map.len()).unwrap();
        name_map.insert(elem.to_string(), len);
        len
    })
}

pub fn generator(input: &str) -> Input {
    let (rules, molecule) = input.split_once("\n\n").unwrap();

    let mut name_map = HashMap::new();

    let mut rules_map: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();
    for line in rules.lines() {
        let (input, output) = line.split_once(" => ").unwrap();
        let input = get_identifier(input, &mut name_map);
        let outputs = split_molecule(output, &mut name_map);
        rules_map.entry(input).or_default().push(outputs);
    }

    let molecule = split_molecule(molecule, &mut name_map);
    Input {
        name_map,
        rules: rules_map,
        molecule,
    }
}

pub fn part1(input: &Input) -> u64 {
    let mut found = HashSet::new();
    for (i, symbol) in input.molecule.iter().enumerate() {
        let Some(rules) = input.rules.get(symbol) else {
            continue;
        };
        for output in rules.iter().cloned() {
            let new_molecule: Vec<_> = input.molecule[..i]
                .iter()
                .copied()
                .chain(output)
                .chain(input.molecule[i + 1..].iter().copied())
                .collect();
            found.insert(new_molecule);
        }
    }

    found.len() as u64
}

/// We want to think about this backwards, i.e. reducing the molecule to e
///
/// This algorithm came from analyzing the input as a grammar. The productions are of three forms:
/// (Let An be any atom except e, Rn, Y, and Ar)
/// Rule 1: e  => A1 A2
/// Rule 2: A1 => A2 A3
/// Rule 3: A1 => A2 Rn (A3 Y){0,2} A4 Ar
///
/// Rule 1 is just a special version of Rule 2, and all the same notes apply.
///   However, it will only be invoked a single time as the final edit
///
/// Each invocation of Rule 2 reduces the length of the molecule by 1.
///   If you had a molecule that could only be reduced by Rule 2, it would take |M| - 1 edits to do so.
///
/// Let's analyze a simpler version of Rule 3 by ignoring the grouping with Y: A1 => A2 Rn A3 Ar.
///   In this version, we can see that each invocation of this rule reduces the length by 3.
///   Rn and Ar both show up only in this rule. So we can simply count the number of times Rn (or Ar) shows up
///   and decrease the required number of edits by 3 for each instance.
///   Since our formula |M| - 1 is already counting the Rn and Ar as normal symbols, we can avoid double counting them by only reducing by 2.
///   Thus we get a new formula |M| - 2*count("Rn") - 1
///
/// Finally, we need to account for the grouping with Y: A1 => A2 Rn (A3 Y){0,2} A4 Ar.
///   We don't actually care that it can only occur two times per rule, so let's simplify to A1 => A2 Rn (A3 Y)* A4 Ar.
///   Every instance of this rule decreases the length by 3 + 2*count(Y), so we simply add in a term to account for the Ys:
///   |M| - 2*count("Rn") - 2*count("Y") - 1
pub fn part2(input: &Input) -> u64 {
    let rn = input.name_map["Rn"];
    let y = input.name_map["Y"];

    let mut edits = input.molecule.len() - 1;
    for &atom in &input.molecule {
        if atom == rn || atom == y {
            edits -= 2;
        }
    }
    edits as u64
}

crate::solution!(
    Input,
    2015,
    19,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
