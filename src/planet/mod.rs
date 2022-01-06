use std::collections::{HashMap, HashSet};

use enum_map::EnumMap;
use nalgebra::Point2;
use rstar::RTree;
use serde::Serialize;

use self::producer::farm::Farm;
use crate::{
    character::CharacterId,
    economy::{Commodity, Ownership, Quality, Quantity},
    pop::Pops,
    terrain::Terrain,
    GalaxyEntityId,
};

pub mod map_view;
mod planet;
mod planet_id;
mod producer;
mod region;
mod region_id;

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
pub enum CrimeKind {
    Theft,
    Arson,
    Assault,
    Murder,
    Fraud,
    Espionage,
}

pub enum LawEnforcementKind {
    Vigilante,
    Military,
    Civilian,
}

pub enum PunishmentKind {
    Subsidized,
    Encouraged,
    None,
    Fine,
    Prison,
    Slavery,
    Death,
}

// i.e. how you view the other person
pub enum Perception {
    Family,
    Dynasty,
    Slave,
    Citizen,
    Foreigner,
    Alien,
    Monster,
}

pub enum Severity {
    None,
    Low,
    Medium,
    High,
    Extreme,
}

pub struct Constitution {
    criminal_law: HashMap<CrimeKind, Severity>,
    enforcement: LawEnforcementKind,
}

pub enum LaborLawKind {}

pub struct Civic {
    labor: LaborLawKind,
}

// Grassland -> Farmland -> Settlement -> Arcology
// Forest -> Glassland (deforestation)
// Forest -> Settlement (elf?)
// Hill -> Mine
// Hill -> Terrace
// Mountain -> Mine
//
pub enum Spot {
    Arcology {
        population: u32,
    },
    Settlement {
        population: u32,
        owned_land: u32,
    },
    // low density
    Farmland {
        population: u32,
        owned_land: u32,
    },
    Mine {
        population: u32,
        owned_land: u32,
    },
    Grassland {
        wildlife_population: u32,
        owned_land: u32,
    },
    Forest {
        wildlife_population: u32,
        owned_land: u32,
    },
    Hill {
        wildlife_population: u32,
        owned_land: u32,
    },
    Terrace {
        population: u32,
        owned_land: u32,
    },
    Mountain {
        wildlife_population: u32,
        owned_land: u32,
    },
}

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

    crime: HashMap<CharacterId, HashMap<CrimeKind, u32>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize)]
pub struct RegionId(usize);

struct RegionIndexData {
    // index of the region in the planet
    index: usize,
    coor: PlanetCoor,
}

pub struct Planet {
    id: GalaxyEntityId,
    name: String,
    regions: Vec<Region>,
    region_index: RTree<RegionIndexData>,
    model_width: f64,
    model_height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanetId(usize);
