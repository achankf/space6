use serde::Serialize;

use crate::{entity::Sequence, planet::Planet, GalaxyEntityId};

mod universe;
mod universe_id;

#[derive(Default)]
pub struct Universe {
    name: String,
    planets: Vec<Planet>,
    entity_idgen: Sequence<GalaxyEntityId>,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub struct UniverseId(usize);

#[derive(Default)]
pub struct Universes {
    data: Vec<Universe>,
}
