use crate::character::Character;
use crate::character::Rarity;
use crate::character::{Ancestry, Background, Class, Stats};
use crate::item::Item;
use crate::item::Slot;
use crate::map::{Color, Location, Pos, Vec2D};
use crate::typed_id::Id;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Movement {
    name: String,
    velocity: Vec2D,
    // TODO: Acceleration?
    // TODO: Rotation?
    // TODO: Animation?
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Condition {
    Not(Box<Condition>),
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Worn(Option<Slot>),
    HasStrength(u32),
    HasWisdom(u32),
    HasConstitution(u32),
    HasDexterity(u32),
    HasIntelligence(u32),
    HasCharisma(u32),
    HasMana(u32),
    HasHealth(u32),
    HasExperiencePoints(u32),
    HasExperienceRate(u32),
    HasMass(u32),
    HasWeightlessness(u32),
    HasSlot(Slot),
    HasAlias(String),
    HasTrait(String),
    IsAncestry(Ancestry),
    IsBackground(Background),
    IsClass(Class),
}

impl Condition {
    pub fn met(&self, stats: &Stats, slot: Option<&Slot>) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurrentEffect {
    Light {
        color: Color,
        radius: u32,
        intensity: u32,
    },
    Sound {
        name: String,
        radius: u32,
        intensity: u32,
    },
    DestroySource,
    DropA(Id<Item>),
    SummonItem(Id<Item>),
    SpawnA(Id<Character>),
    SummonThe(Id<Character>),
    Move(Id<Movement>),
    TeleportTo(Id<Character>),
    TeleportInto(Id<Location>, Pos),
    Buff(String, Stats),
    DeBuff(String, Stats), // Takes away if has.
}

impl CurrentEffect {
    fn apply(&self, stats: &mut Stats, slot: Option<&Slot>) {
        match self {
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimedEffect {
    Current(CurrentEffect),
    Temporary {
        seconds: u32,
        effect: Box<CurrentEffect>,
        on_expiry: Box<CurrentEffect>,
        started: Option<Duration>, // gametime (must be sync'd for multi-play)
        effect_every: Option<u32>,
        effected_last: Option<u32>,
    },
}

impl TimedEffect {
    fn apply(&self, stats: &mut Stats, slot: Option<&Slot>) {
        match self {
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Effect {
    None,
    Immediate(TimedEffect),
    Multiple(Vec<Effect>),
    If {
        condition_name: Option<String>,
        condition: Condition,
        effect: Box<Effect>,
        failed_effect: Box<Effect>,
    },
}

impl Effect {
    pub fn apply(&self, stats: &mut Stats, slot: Option<&Slot>) {
        match self {
            Effect::None => {}
            Effect::Immediate(eff) => {
                eff.apply(stats, slot);
            }
            Effect::Multiple(es) => {
                for eff in es {
                    eff.apply(stats, slot);
                }
            }
            Effect::If {
                condition_name: _,
                condition,
                effect,
                failed_effect,
            } => {
                if condition.met(stats, slot) {
                    effect.apply(stats, slot);
                } else {
                    failed_effect.apply(stats, slot);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Relationship {
    name: String,
    familial: bool,
    work: bool,
    superior: bool,
    subordinate: bool,
    ancestry: Option<Ancestry>,
    class: Option<Class>,
    rarity: Rarity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StoryPointInfo {
    Location(Id<Location>),
    Relationship(Relationship, Id<Character>),
    Effect(Id<Character>, Box<CurrentEffect>),
    Loss(Box<StoryPoint>),
    Job(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StoryPoint {
    pub what: StoryPointInfo,
    pub ordering: Duration, // gametime (must be sync'd for multi-play)
    pub actor: Option<Id<Character>>,
    pub location: Option<Id<Location>>,
    pub effect: Effect,
}
