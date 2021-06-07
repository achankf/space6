use crate::{
    planet::{PlanetCoor, Region, RegionId},
    terrain::Terrain,
};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct RegionFacade {
    centroid_coor: PlanetCoor,
    border_vertices: Vec<PlanetCoor>,
    neighbours: HashSet<RegionId>,
    pub noise: f64,
    pub terrain: Terrain,
}

impl RegionFacade {
    pub fn get_centroid_coor(&self) -> PlanetCoor {
        self.centroid_coor
    }
}

#[wasm_bindgen]
impl RegionFacade {
    pub fn get_centroid_x(&self) -> f64 {
        self.centroid_coor.x
    }

    pub fn get_centroid_y(&self) -> f64 {
        self.centroid_coor.y
    }

    /// Every 2 items in the result form a (x,y) coordinate pair
    pub fn get_border_vertices(&self) -> Vec<f64> {
        self.border_vertices
            .iter()
            .flat_map(|vertex| [vertex.x, vertex.y])
            .collect()
    }

    pub fn number_of_border_verticies(&self) -> usize {
        self.border_vertices.len()
    }
}

impl From<Region> for RegionFacade {
    fn from(
        Region {
            centroid_coor,
            border_vertices,
            neighbours,
            noise,
            terrain,
            ..
        }: Region,
    ) -> Self {
        Self {
            centroid_coor,
            border_vertices,
            neighbours,
            noise,
            terrain,
        }
    }
}
