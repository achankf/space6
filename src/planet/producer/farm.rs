use std::collections::HashSet;

use crate::{
    character::CharacterId,
    economy::{Ownership, Wage},
    planet::LandSize,
    pop::PopCount,
};

#[derive(Clone)]
pub struct Farm {
    workers: HashSet<CharacterId>,
    wage_rate: Wage,
    employed: PopCount,
    ownership: Ownership,
    owned_land: LandSize,
}
