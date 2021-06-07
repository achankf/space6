use enum_map::{Enum, EnumMap};

use crate::Time;

mod brand;
mod company;

#[derive(Enum, Clone)]
pub enum Commodity {
    // food
    Crop, // from farms and water; cereal, fruit, etc.
    Meat, // from pastures or fisheries, all flesh from animals
    // food flavoring
    Salt,    // mined from salt pan (found in desert)
    Essence, // extracted from highly magical plants or synthesized from Chemical
    Spice,   // mined from only 1 planet that filled with aggressive giant death sand worms
    // drink
    Water,    // from various ways based on Tech and energy
    Beverage, // from crop

    // raw materials
    Chemical, // from Crop or Metal
    Fiber,    // from Crop
    Polymer,  // from Wood, FossilFuel, Crop, recycled waste
    Glass,    // from nothing; assuming sand is free
    Incense,

    Fuel,

    // minerals
    Gold,  // I, has special economic mechanisms
    Gem,   // I, to jewelry or magic storage
    Metal, // I, represents most metal found on Earth
    Alloy, // I, represents alloys from Metals

    // products
    Apparel,   // from fiber
    Paper,     // from wood, obsolete in late-game
    Accessory, // from wood or gems
    Furniture, // from wood or parts
    Appliance, // from Parts, or (Circuit | Bot) & polymers
    Gadget,    // from (Circuit | Bot) & polymers
    Vehicle,   // from parts & polymers
    Medicine,  // from Herbs, Chemicals, or Bots

    // space travel
    Stillsuit, // from polymer and fiber

    // usable products for manufacturing
    Fertilizer, // from Animal manure & chemicals
    Tool,       // from metal or parts
    Machine,    // from parts
    Computer,
    Printer, // from (Circuit | Bot) & polymers

    // replaceable parts / advanced materials
    Plate, // from alloy; gameplay-wise to consolidate metal resources
    Parts, // from wood, metal, alloy
    // construction
    Scaffold,  // from wood
    Structure, // from Plate
}

pub type Quality = u32;
pub type Quantity = u32;

pub struct Product {
    commodity: Commodity,
    quality: Quality,
}

pub struct Company {
    brands: EnumMap<Commodity, BrandId>,
}

#[derive(Clone)]
pub struct CompanyId(usize);

pub struct Brand {
    created_at: Time,
}

#[derive(Clone)]
pub struct BrandId(usize);

#[derive(Clone)]
pub enum Ownership {
    Company(CompanyId), // contributes to warehouses, then local market, then discard remaining
    Independent,        // contributes to local market
}

#[derive(Clone)]
pub enum Wage {
    Low,
    Normal,
    High,
}

// ref:
// https://secure.math.ubc.ca/~malabika/teaching/ubc/spring11/math105/surplus.pdf
