use crate::{Sample, samples};
use itertools::Itertools;

type Input = Fighter;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

const SHOP: &str = r"
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

const PLAYER_HP: u32 = 100;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Fighter {
    pub hp: u32,
    pub damage: u32,
    pub armor: u32,
}

impl Fighter {
    fn turns_to_defeat(&self, defender: &Self) -> u32 {
        let damage_per_turn = self.damage.saturating_sub(defender.armor).max(1);
        defender.hp.div_ceil(damage_per_turn)
    }

    pub fn will_defeat(&self, defender: &Self) -> bool {
        self.turns_to_defeat(defender) <= defender.turns_to_defeat(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

fn shop_combinations() -> impl Iterator<Item = (u32, u32, u32)> {
    static ITEM_PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

    let mut sections = SHOP.split("\n\n").map(|section| {
        section.lines().skip(1).filter_map(|line| {
            let item_pattern = ITEM_PATTERN.get_or_init(|| {
                regex::Regex::new(r"^([A-Za-z+\d ]+?)\s+(\d+)\s+(\d+)\s+(\d+)$").unwrap()
            });

            let caps = item_pattern.captures(line)?;
            let name = &caps[1];
            let cost = caps[2].parse::<u32>().unwrap();
            let damage = caps[3].parse::<u32>().unwrap();
            let armor = caps[4].parse::<u32>().unwrap();
            Some(Item {
                name: name.to_string(),
                cost,
                damage,
                armor,
            })
        })
    });
    let weapons = sections.next().unwrap();

    let armor = sections.next().unwrap().chain(std::iter::once(Item {
        name: "No Armor".to_string(),
        cost: 0,
        damage: 0,
        armor: 0,
    }));
    let rings: Vec<_> = sections.next().unwrap().collect();
    let rings = rings
        .clone()
        .into_iter()
        .combinations(2)
        .chain(rings.into_iter().map(|ring| vec![ring.clone()]))
        .chain(std::iter::once(Vec::new()));

    weapons
        .cartesian_product(armor)
        .cartesian_product(rings)
        .map(|((weapon, armor), rings)| {
            [weapon, armor].into_iter().chain(rings).fold(
                (0_u32, 0_u32, 0_u32),
                |(cost, damage, armor), item| {
                    (cost + item.cost, damage + item.damage, armor + item.armor)
                },
            )
        })
}

pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let hp: u32 = prse::parse!(lines.next().unwrap(), "Hit Points: {}");
    let damage: u32 = prse::parse!(lines.next().unwrap(), "Damage: {}");
    let armor: u32 = prse::parse!(lines.next().unwrap(), "Armor: {}");
    Fighter { hp, damage, armor }
}

pub fn part1(input: &Input) -> u64 {
    shop_combinations()
        .filter_map(|(total_cost, total_damage, total_armor)| {
            let player = Fighter {
                hp: PLAYER_HP,
                damage: total_damage,
                armor: total_armor,
            };
            player.will_defeat(input).then_some(total_cost)
        })
        .min()
        .unwrap()
        .into()
}

pub fn part2(input: &Input) -> u64 {
    shop_combinations()
        .filter_map(|(total_cost, total_damage, total_armor)| {
            let player = Fighter {
                hp: PLAYER_HP,
                damage: total_damage,
                armor: total_armor,
            };
            (!player.will_defeat(input)).then_some(total_cost)
        })
        .max()
        .unwrap()
        .into()
}

crate::solution!(
    Input,
    2015,
    21,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
