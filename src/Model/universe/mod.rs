use crate::{entity::Sequence, planet::Planet, GalaxyEntityId};
use serde::Serialize;

mod universe;
mod universe_id;

#[derive(Default)]
pub struct Universe {
    name: String,
    planets: Vec<Planet>,
    entity_idgen: Sequence<GalaxyEntityId>,
}

#[derive(Clone, Copy, Serialize)]
pub struct UniverseId(usize);

#[derive(Default)]
pub struct Universes {
    pub data: Vec<Universe>,
}
