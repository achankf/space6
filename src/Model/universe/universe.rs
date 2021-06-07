use crate::{
    entity::Sequence,
    planet::{Planet, PlanetId},
};

use super::{Universe, UniverseId, Universes};

impl Universe {
    pub fn clone_name(&self) -> String {
        self.name.to_owned()
    }

    fn create(creation_rng: &mut dyn rand_core::RngCore) -> Self {
        let entity_idgen = Sequence::default();

        let name = format!("{} {}", "Main", creation_rng.next_u32());
        Self {
            name,
            planets: Default::default(),
            entity_idgen,
        }
    }

    pub fn create_planet(
        &mut self,
        creation_rng: &mut dyn rand_core::RngCore,
    ) -> (PlanetId, &mut Planet) {
        let mut planet_global_idgen = Sequence::default();
        let planet = Planet::new(creation_rng, planet_global_idgen.next());
        let planet_id = self.planets.len();
        self.planets.push(planet);
        (
            PlanetId::new_unsafe(planet_id),
            &mut self.planets[planet_id],
        )
    }

    pub fn get_planet_unsafe(&self, index: usize) -> &Planet {
        &self.planets[index]
    }

    pub fn get_planet(&self, index: PlanetId) -> &Planet {
        let index: usize = From::from(index);
        &self.planets[index]
    }

    pub fn count_planets(&self) -> usize {
        self.planets.len()
    }
}

impl Universes {
    pub fn create(
        &mut self,
        creation_rng: &mut dyn rand_core::RngCore,
    ) -> (UniverseId, &mut Universe) {
        let universe = Universe::create(creation_rng);
        let data = &mut self.data;
        let id = data.len();
        data.push(universe);
        (id.into(), &mut data[id])
    }
}
