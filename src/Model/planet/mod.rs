use self::producer::farm::Farm;
use crate::character::CharacterId;
use crate::economy::Commodity;
use crate::economy::Ownership;
use crate::economy::Quality;
use crate::economy::Quantity;
use crate::pop::Pops;
use crate::terrain::Terrain;
use crate::GalaxyEntityId;
use enum_map::EnumMap;
use nalgebra::Point2;
use rstar::RTree;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

mod canvas_wasm;
mod planet;
mod planet_id;
pub mod planet_wasm;
mod producer;
mod region;
mod region_id;
pub mod region_wasm;

pub type LandSize = u16;

/*
Zoning determines how much land is allocated for a paricular purpose in a region:
- farming
- industry
- commercial
- residence
- special
*/
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RegionSpecialty {
    // natural
    Uncivilized,

    // pre-modern
    Rural,    // mostly farming
    Forestry, // for chopping wood while preseving forests for industrial uses
    Mining,   // can be place anyway that has a mineral deposit
    Estate,   // allow more characters to live; mostly residence & some commercial
    Arcane,   // university, martial sect, Moria, and any fantasy shit here

    // modern
    Urban, // mostly high-density residental & commerical; factories are allowed but undesirable
    RocketPad, // place for launching rockets, or for docking for reusable ones
    Industry, // dense industrial complex
    DataCenter, // an energy-efficient place that has lots of computers
    SpaceElevator, // able to move lots of goods & people out to space
}

impl Default for RegionSpecialty {
    fn default() -> Self {
        RegionSpecialty::Uncivilized
    }
}

#[derive(Clone)]
pub struct RetailerStorage {
    avg_quality: u32,
    avg_brand: u32,
    quantity: Quantity,
}

// rating = weighted sum of (quality + brand + price rating)
// rating affects market penetration

// price rating = "intrinsic price" / retail price - 1, with adjustments

// "intrinsic price" is a constant + factors due to planet/galaxy events

// equilibrium price = "intrinsic price" * (demand / supply)^elasticity

// demand = % of market share * total pop consumption

// % of market share = portion of market penetration to the regional total

/**
i.e. permanently stationed stores
*/
#[derive(Clone)]
pub struct Retailer {
    manager: Option<CharacterId>,
    // how many physical stores -> coverage
    num_stores: u32,
    storage: EnumMap<Commodity, RetailerStorage>,
}

impl Retailer {
    pub fn cal_market_penetration(&self) -> u32 {
        0
    }
}

#[derive(Clone, Default)]
pub struct Storage(EnumMap<Commodity, Quantity>);

#[derive(Clone)]
pub struct Warehouse {
    scale: u32,
    quality: Quality,
    storage: Storage,
}

/// Basically Port Royale style markets, which everyone can trade freely.
/// Represents "independent" retailers, travelling merchants, etc.
/// Goods sell after retail
#[derive(Default, Clone)]
pub struct LocalMarket {
    scale: u32,
    storage: Storage,
}

pub type PlanetCoor = Point2<f64>;

#[derive(Clone)]
pub struct Region {
    centroid_coor: PlanetCoor,
    border_vertices: Vec<PlanetCoor>,
    neighbours: HashSet<RegionId>,
    noise: f64,
    terrain: Terrain,

    specialty: RegionSpecialty,
    stability: u32,

    local_market: LocalMarket,

    farms: HashMap<Ownership, Farm>,

    pops: Pops,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub struct RegionId(usize);

struct RegionIndexData {
    // index of the region in the planet
    index: usize,
    coor: PlanetCoor,
}

#[wasm_bindgen]
pub struct Planet {
    id: GalaxyEntityId,
    name: String,
    regions: Vec<Region>,
    region_index: RTree<RegionIndexData>,
    model_width: f64,
    model_height: f64,
}

#[derive(Clone, Copy, Serialize)]
pub struct PlanetId(usize);
