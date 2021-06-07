use character::Character;
use entity::Succ;
use planet::PlanetId;
use planet::RegionId;
use serde::Serialize;
use unit::Parties;
use universe::UniverseId;
use universe::Universes;
use wasm_bindgen::prelude::*;

mod character;
mod coor;
mod economy;
mod entity;
pub mod model_wasm;
mod planet;
mod pop;
mod terrain;
mod unit;
mod universe;
mod util;

pub type Time = u64;

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct GalaxyEntityId(usize);

impl Succ for GalaxyEntityId {
    fn succ(self) -> Self {
        Self(self.0.succ())
    }
}

pub enum Deposit {
    Soil,
    Mineral,
    Tree,
}

pub type Coor = nalgebra::Point2<f64>;

#[derive(Clone, Copy, Serialize)]
pub enum CompleteCoor {
    OnPlanetRoad(UniverseId, PlanetId, Coor),
    OnPlanetRegion(UniverseId, PlanetId, RegionId),
    InSpace(UniverseId, Coor),
}

pub struct Game {
    tick: u64,
    characters: Vec<Character>,
    universes: Universes,
    parties: Parties,
    player_characcter_id: usize,
}

impl Game {
    pub fn progress(&mut self) {
        self.tick += 1;
    }
}

#[wasm_bindgen]
pub struct Model {
    game: Game,
}
