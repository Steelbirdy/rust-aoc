use crate::{Sample, aoc2015::day21::Fighter, samples};

type Input = Fighter;

const PART1_CHECKS: &[Sample] = samples![];

const PART2_CHECKS: &[Sample] = samples![];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub const ALL: [Self; 5] = [
        Self::MagicMissile,
        Self::Drain,
        Self::Shield,
        Self::Poison,
        Self::Recharge,
    ];

    pub fn cost(self) -> u32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    pub fn cast(self, game: &mut GameState) {
        match self {
            Spell::MagicMissile => game.cast_magic_missile(),
            Spell::Drain => game.cast_drain(),
            Spell::Shield => game.cast_shield(),
            Spell::Poison => game.cast_poison(),
            Spell::Recharge => game.cast_recharge(),
        }
    }

    pub fn can_cast(self, game: &GameState) -> bool {
        if game.result.is_some() {
            return false;
        }
        match self {
            Self::MagicMissile => game.wizard.mp >= 53,
            Self::Drain => game.wizard.mp >= 73,
            Self::Shield => game.wizard.mp >= 113 && game.shield_duration <= 1,
            Self::Poison => game.wizard.mp >= 173 && game.poison_duration <= 1,
            Self::Recharge => game.wizard.mp >= 229 && game.recharge_duration <= 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Wizard {
    pub hp: u32,
    pub armor: u32,
    pub mp: u32,
    pub total_mp_used: u32,
}

impl Wizard {
    fn use_mp(&mut self, mp: u32) {
        self.mp -= mp;
        self.total_mp_used += mp;
    }
}

impl Default for Wizard {
    fn default() -> Self {
        Self {
            hp: 50,
            armor: 0,
            mp: 500,
            total_mp_used: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameState {
    wizard: Wizard,
    boss: Fighter,
    shield_duration: u32,
    poison_duration: u32,
    recharge_duration: u32,
    result: Option<bool>,
}

impl GameState {
    fn damage_wizard(&mut self, damage: u32) -> u32 {
        let damage = damage.saturating_sub(self.wizard.armor).max(1);
        self.wizard.hp = self.wizard.hp.saturating_sub(damage);
        if self.wizard.hp == 0 {
            self.result = Some(false);
        }
        damage
    }

    fn damage_boss(&mut self, damage: u32) -> u32 {
        self.boss.hp = self.boss.hp.saturating_sub(damage);
        if self.boss.hp == 0 {
            self.result = Some(true);
        }
        damage
    }

    fn turn_start(&mut self) {
        match self.shield_duration {
            0 => {}
            1 => {
                self.shield_duration = 0;
                self.wizard.armor = 0;
            }
            2.. => {
                self.shield_duration -= 1;
            }
        }

        if self.poison_duration > 0 {
            self.damage_boss(3);
            self.poison_duration -= 1;
        }

        if self.recharge_duration > 0 {
            self.wizard.mp += 101;
            self.recharge_duration -= 1;
        }
    }

    fn boss_turn(&mut self) {
        self.turn_start();
        if self.result.is_some() {
            return;
        }
        self.damage_wizard(self.boss.damage);
    }

    fn wizard_turn(&mut self, spell: Spell) {
        assert!(spell.can_cast(self));

        self.turn_start();
        if self.result.is_some() {
            return;
        }
        self.wizard.use_mp(spell.cost());
        spell.cast(self)
    }

    fn cast_magic_missile(&mut self) {
        self.damage_boss(4);
    }

    fn cast_drain(&mut self) {
        self.wizard.hp += 2;
        self.damage_boss(2);
    }

    fn cast_shield(&mut self) {
        self.shield_duration = 6;
        self.wizard.armor = 7;
    }

    fn cast_poison(&mut self) {
        self.poison_duration = 6;
    }

    fn cast_recharge(&mut self) {
        self.recharge_duration = 5;
    }

    fn gather_options(self) -> impl DoubleEndedIterator<Item = Spell> {
        let game = self.clone();
        Spell::ALL
            .into_iter()
            .filter(move |spell| spell.can_cast(&game))
    }

    fn successors(self) -> impl Iterator<Item = (Self, u32)> {
        let game = self;
        game.clone().gather_options().map(move |spell| {
            let mut game = game.clone();
            game.wizard_turn(spell);
            if game.result.is_none() {
                game.boss_turn();
            }
            (game, spell.cost())
        })
    }
}

pub fn generator(input: &str) -> Input {
    let mut lines = input.lines();
    let hp: u32 = prse::parse!(lines.next().unwrap(), "Hit Points: {}");
    let damage: u32 = prse::parse!(lines.next().unwrap(), "Damage: {}");
    Fighter {
        hp,
        damage,
        armor: 0,
    }
}

pub fn part1(input: &Input) -> u64 {
    let game = GameState {
        wizard: Wizard::default(),
        boss: *input,
        shield_duration: 0,
        poison_duration: 0,
        recharge_duration: 0,
        result: None,
    };

    let (_, result) = crate::utils::dijkstra(
        game,
        |game| {
            let iter = if game.result.is_some() {
                None
            } else {
                Some(game.clone().successors())
            };
            iter.into_iter().flatten()
        },
        |game| game.result == Some(true),
    )
    .unwrap();

    result.into()
}

pub fn part2(input: &Input) -> u64 {
    let mut game = GameState {
        wizard: Wizard::default(),
        boss: *input,
        shield_duration: 0,
        poison_duration: 0,
        recharge_duration: 0,
        result: None,
    };

    game.wizard.hp -= 1;
    game.boss.damage += 1;

    let (_, result) = crate::utils::dijkstra(
        game,
        |game| {
            let iter = if game.result.is_some() {
                None
            } else {
                Some(game.clone().successors())
            };
            iter.into_iter().flatten()
        },
        |game| game.result == Some(true),
    )
    .unwrap();

    result.into()
}

crate::solution!(
    Input,
    2015,
    22,
    generator,
    part1,
    PART1_CHECKS,
    part2,
    PART2_CHECKS
);
