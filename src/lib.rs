use character::Character;
use dynasty::Dynasty;
use entity::Succ;
use log::Level;
use planet::{PlanetId, RegionId};
use unit::Parties;
use universe::{UniverseId, Universes};
use wasm_bindgen::prelude::*;

mod app;
mod character;
mod coor;
mod dynasty;
mod economy;
mod entity;
mod faction;
mod game;
mod id;
mod planet;
mod pop;
mod terrain;
mod unit;
mod universe;
pub mod util;

pub type Time = u64;

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct GalaxyEntityId(usize);

impl Succ for GalaxyEntityId {
    fn succ(self) -> Self {
        Self(self.0.succ())
    }
}

pub type Coor = nalgebra::Point2<f64>;

#[derive(Clone, Copy)]
pub enum CompleteCoor {
    OnPlanetRoad(UniverseId, PlanetId, Coor),
    OnPlanetRegion(UniverseId, PlanetId, RegionId),
    InSpace(UniverseId, Coor),
}

pub struct Game {
    tick: u64,
    characters: Vec<Character>,
    universes: Universes,
    dynasties: Vec<Dynasty>,
    parties: Parties,
    player_character_id: usize,

    // gets incremented whenever side-effects to the object happen
    generation: u64,
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Debug).expect("error initializing logger");
    console_error_panic_hook::set_once();
    yew::start_app::<crate::app::main::App>();
}
