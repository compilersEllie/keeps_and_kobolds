use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

use crate::actions::{Action, Discussion};
use crate::effects::{Condition, Effect, StoryPoint};
use crate::item::{Item, Slot};
use crate::map::Location;
use crate::typed_id::Id;

type Centimetres = u32;
type Years = u32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    OnePerGroup, // e.g. King, Queen, Boss, Lich
    Unique,      // Never randomly generated
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ancestry {
    // TODO: Add more ancestries.
    // TODO: Register from files.
    name: String,
    stats: Stats,
    rarity: Rarity,
    nonplayable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Background {
    // TODO: Add more backgrounds.
    // TODO: Register from files.
    name: String,
    stats: Stats,
    rarity: Rarity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Class {
    // TODO: Add more classes.
    // TODO: Register from files.
    name: String,
    stats: Stats,
    rarity: Rarity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Target {
    AnyLiving,
    AnyOtherSpecies,
    AnyOtherBackground,
    AnyWeaker,
    Character,
    Player,
    OtherCharacter(Id<Character>),
    Item(Id<Item>),
    Location(Id<Location>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Goal {
    target: Target,
    action: Action,
    condition: Option<Condition>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stats {
    pub strength: u32,
    pub wisdom: u32,
    pub constitution: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub charisma: u32,
    pub mana: u32,
    pub health: u32,
    pub experience_points: u32,
    pub mass: u32,

    pub gravity_percent: u32,
    pub experience_rate_percent: u32,
    pub weight: u32,

    pub min_height: Option<Centimetres>,
    pub height: Centimetres,
    pub max_height: Option<Centimetres>,
    pub age: Years,
    pub max_age: Option<Years>,
    pub life_extension: Years,

    pub slots: Vec<Slot>,
    pub equipped: Vec<(Slot, Item)>,
    pub actions: Vec<Action>,
    pub effects: Vec<Effect>,
    pub aliases: Vec<String>,
    pub traits: Vec<String>,
    pub goals: Vec<Goal>,
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        // Basic stats
        self.strength += other.strength;
        self.wisdom += other.wisdom;
        self.constitution += other.constitution;
        self.dexterity += other.dexterity;
        self.intelligence += other.intelligence;
        self.charisma += other.charisma;
        self.mana += other.mana;
        self.health += other.health;
        self.mass += other.mass;
        self.height += other.height;
        self.age += other.age;
        // Multiplicative
        self.experience_rate_percent += other.experience_rate_percent;
        self.gravity_percent += other.gravity_percent;

        // Collections
        self.slots.extend(other.slots);
        self.equipped.extend(other.equipped);
        self.actions.extend(other.actions);
        self.effects.extend(other.effects);
        self.aliases.extend(other.aliases);
        self.traits.extend(other.traits);
        self.goals.extend(other.goals);

        // Limits
        self.min_height = self
            .min_height
            .iter()
            .chain(&other.min_height)
            .max()
            .copied();
        self.max_height = self
            .max_height
            .iter()
            .chain(&other.max_height)
            .min()
            .copied();
        self.max_age = self.max_age.iter().chain(&other.max_age).min().copied();

        // Specials
        self.weight = ((self.weight as f32) * (other.gravity_percent as f32) / 100.0) as u32;
        self.experience_points += ((self.experience_rate_percent as f32)
            * (other.experience_points as f32)
            / 100.0) as u32;
        if let Some(max_age) = &mut self.max_age {
            *max_age += self.life_extension;
            *max_age += other.life_extension;
            self.life_extension = 0;
        } else {
            self.life_extension += other.life_extension;
        }
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Stats) -> Stats {
        let mut res = self.clone();
        res += other;
        res
    }
}

impl Stats {
    fn new() -> Stats {
        Stats::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Character {
    // TODO: Implement character
    name: String, // Primary
    ancestry: Ancestry,
    background: Background,
    class: Class,

    // Story changes, NPC & AFK
    history: Vec<StoryPoint>,
    lines: Discussion,

    // Cached
    stats: Option<Stats>,
}

impl Character {
    fn compute(&mut self) -> Stats {
        if let Some(stats) = &self.stats {
            return stats.clone();
        }
        let mut stats = Stats::new();
        stats += self.ancestry.stats.clone();
        stats += self.background.stats.clone();
        stats += self.class.stats.clone();

        for storypoint in &self.history {
            storypoint.effect.apply(&mut stats, None);
        }

        for (slot, item) in &stats.equipped.clone() {
            for storypoint in &item.history {
                storypoint.effect.apply(&mut stats, Some(&slot));
            }
        }
        self.stats = Some(stats.clone());
        stats
    }

    // TODO: Calculate get situational actions
    // TODO: Movement modes
    // TODO: Stat tests
    // TODO: Character creator
    // TODO: Vision stat
}
