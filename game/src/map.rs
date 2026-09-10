use crate::asset;
use crate::character::Character;
use crate::item::Item;
use crate::typed_id::Id;
use noise::{HybridMulti, NoiseFn, Perlin};
use polymap::PolyMap;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::collections::HashMap;
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

asset!(Location);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TileKind {
    name: String,
    tiles: Vec<(char, Color)>, // TODO(feat): Add textures images
}

asset!(TileKind);

// TODO: Implement map #1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Map {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tilekinds: Vec<Id<TileKind>>,
    #[serde(default)]
    pub tiles: Vec<(char, Color)>,

    pub up: Option<Id<Location>>,
    pub down: Option<Id<Location>>,
    pub left: Option<Id<Location>>,
    pub right: Option<Id<Location>>,

    pub items: Vec<(Pos, Id<Item>)>,
    pub characters: Vec<(Pos, Id<Character>)>,
}

impl Map {
    pub fn generate(&mut self, asset_store: &PolyMap<TypeId>) {
        let n = HybridMulti::<Perlin>::new(1234u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;

                // -1..1
                let tile_n = n.get([x as f64, y as f64]);
                let tile_h = ((self.tilekinds.len() - 1) as f64) * (tile_n + 1.0) / 2.0;
                let tile_k = tile_h as usize;
                let tilekind_id = &self.tilekinds[tile_k];

                let tilekind = asset_store
                    .get::<TypeId, HashMap<Id<TileKind>, TileKind>>(&TypeId::of::<TileKind>())
                    .unwrap_or_else(|| panic!("TileKind isn't loaded"))
                    .get(tilekind_id)
                    .unwrap_or_else(|| panic!("TileKind {:?} isn't loaded", tilekind_id));
                let tile_sn = n.get([x as f64, y as f64]);
                let tile_sh = ((tilekind.tiles.len() - 1) as f64) * (tile_n + 1.0) / 2.0;
                let tile_t = tile_sh as usize;
                let tile = tilekind.tiles[tile_t];
                self.tiles.push(tile);
            }
        }
    }
}

asset!(Map);

// TODO(feat): Map generation via continuous noise functions #3
// e.g. https://docs.isaratech.com/ue4-plugins/noise-library/generators/ridged-multi
//      Height map - Octave count = 12
//      Maybe Water ways - Spectral weights exponent = 2
// TODO(feat): Use erosion modeling https://stackoverflow.com/questions/36796829/procedural-terrain-with-ridged-fractal-noise #5
// More inspiration https://www.world-machine.com/features.php#feature-simulation
