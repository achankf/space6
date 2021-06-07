use super::region_wasm::RegionFacade;
use crate::planet::Planet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct PlanetMapFacade {
    data: Vec<RegionFacade>,
    name: String,
    pub model_width: f64,
    pub model_height: f64,
}

impl PlanetMapFacade {
    pub fn from(planet: &Planet) -> Self {
        let data = planet
            .get_regions()
            .iter()
            .cloned()
            .map(Into::into)
            .collect();
        let name = planet.clone_name();

        PlanetMapFacade {
            data,
            name,
            model_width: planet.model_width,
            model_height: planet.model_height,
        }
    }
}

#[wasm_bindgen]
impl PlanetMapFacade {
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}
