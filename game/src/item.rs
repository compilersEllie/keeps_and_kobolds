use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::effects::StoryPoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemKind {
    Passive,             // Jewelry, Clothes, Armour, Coins, etc.
    Tool(Vec<Action>),   // Actions
    Weapon(Vec<Action>), // Actions
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Handedness {
    Left(Option<u8>),
    Right(Option<u8>),
    Dual, // Complicated as it interacts with L/R.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Slot {
    // Humanoid default
    Arm(Handedness),
    Shoulder(Handedness),
    Hand(Handedness),
    HandHeld(Handedness),
    Finger(Handedness),
    Thumb(Handedness),
    Wrist(Handedness),
    Leg(Handedness),
    Ankle(Handedness),
    Foot(Handedness),
    Toe(Handedness),
    Head(Option<u8>),
    Eye(Handedness),
    Neck(Option<u8>),
    Jaw(Option<u8>),  // e.g. Equip custom teeth
    Hair(Option<u8>), // e.g. styles and colour

    // Non human parts:
    Tail(Option<u8>),
    Horn(Option<Handedness>),
    Tentacle(Option<u8>),
    Wing(Handedness),
    ChestCavity,

    // Multiple of the above slots are possible (e.g. Centaurs,some Demons and Gargoyles)

    // Body parts that are typically unique and common to all ancestries.
    UpperBody,
    LowerBody,
    Waist,
    Back,          // Bags, Shields, etc.
    BackSecondary, // Quivers, Doublehanded weapons, etc.

    // For thief / gremlin behaviours like eating an item.
    Stomach,
    PrisonWallet, // ?
    Mouth(Option<u8>),

    // Support for amputees to equip prostheses.
    ResidualArm(Option<u8>),
    ResidualLeg(Option<u8>),

    // Attachments:
    Quiver,
    Bag,
    CoinPurse,
    Belt,

    // Item provided slots:
    Scabbard,
    Enchantment(Option<u8>),
    Pocket,
    ConcealedPocket,
    Necklace(Option<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Item {
    pub name: Option<String>, // Named objects exist.
    pub kind: ItemKind,
    pub slot: Slot,

    // Creates effects on the user etc.
    pub history: Vec<StoryPoint>,
}
