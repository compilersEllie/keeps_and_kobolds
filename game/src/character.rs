use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

use crate::actions::{Action, Discussion};
use crate::effects::{Condition, Effect, StoryPoint};
use crate::item::{Item, Slot};
use crate::map::Location;
use crate::typed_id::Id;

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

type Centimetres = u32;
type Years = u32;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    OnePerGroup, // e.g. King, Queen, Boss, Lich
    Unique,      // Never randomly generated
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ancestry {
    // TODO: Add more ancestries. #4
    // TODO: Register from files. #2
    name: String,
    stats: Stats,
    #[serde(skip_serializing_if = "is_default")]
    rarity: Rarity,
    #[serde(skip_serializing_if = "is_default")]
    nonplayable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Background {
    // TODO: Add more backgrounds. #3
    // TODO: Register from files. #2
    name: String,
    stats: Stats,
    rarity: Rarity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Class {
    // TODO: Add more classes. #2
    // TODO: Register from files. #2
    name: String,
    stats: Stats,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    rarity: Rarity,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Target {
    #[default]
    AnyOtherSpeciesOrBackground,
    AnyOtherSpecies,
    AnyOtherBackground,
    AnyLiving,
    AnyWeaker,
    Character,
    Player,
    OtherCharacter(Id<Character>),
    Item(Id<Item>),
    Location(Id<Location>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Goal {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    target: Target,
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(default)]
pub struct Stats {
    #[serde(skip_serializing_if = "is_default")]
    pub strength: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub wisdom: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub constitution: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub dexterity: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub intelligence: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub charisma: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub mana: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub health: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub experience_points: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub mass: u32,

    #[serde(skip_serializing_if = "is_default")]
    pub gravity_percent: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub experience_rate_percent: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub weight: u32,

    #[serde(skip_serializing_if = "is_default")]
    pub height: Centimetres,
    #[serde(skip_serializing_if = "is_default")]
    pub age: Years,
    #[serde(skip_serializing_if = "is_default")]
    pub life_extension: Years,

    pub min_height: Option<Centimetres>,
    pub max_height: Option<Centimetres>,
    pub max_age: Option<Years>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<Slot>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub equipped: Vec<(Slot, Item)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    // TODO: Implement character #2
    name: String, // Primary
    ancestry: Ancestry,
    background: Background,
    class: Class,

    // Story changes, NPC & AFK
    history: Vec<StoryPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<Discussion>,

    // Cached
    #[serde(skip)]
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
                storypoint.effect.apply(&mut stats, Some(slot));
            }
        }
        self.stats = Some(stats.clone());
        stats
    }

    // TODO: Calculate get situational actions #2
    // TODO: Movement modes #2
    // TODO: Stat tests #3
    // TODO: Character creator #3
    // TODO: Vision stat #2
}
