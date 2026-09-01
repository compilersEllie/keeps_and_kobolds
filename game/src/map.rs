use crate::character::Character;
use crate::item::Item;
use crate::typed_id::Id;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    fn add(self, other: Vec2D) -> Self {
        Self {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

#[derive(
    Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Vec2D {
    pub dx: u32,
    pub dy: u32,
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location {
    pub name: String,
    pub nickname: Option<String>,
    pub description: String,
}

// TODO: Implement map
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<()>,

    pub up: Option<Id<Location>>,
    pub down: Option<Id<Location>>,
    pub left: Option<Id<Location>>,
    pub right: Option<Id<Location>>,

    pub items: Vec<(Pos, Id<Item>)>,
    pub character: Vec<(Pos, Id<Character>)>,
}

// TODO(feat): Map generation via continuous noise functiond
// e.g. https://docs.isaratech.com/ue4-plugins/noise-library/generators/ridged-multi
//      Height map - Octave count = 12
//      Maybe Water ways - Spectral weights exponent = 2
// TODO(feat): Use erosion modeling https://stackoverflow.com/questions/36796829/procedural-terrain-with-ridged-fractal-noise
// More inspiration https://www.world-machine.com/features.php#feature-simulation
