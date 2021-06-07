use crate::{
    character::Character,
    planet::{planet_wasm::PlanetMapFacade, PlanetCoor},
    universe::Universes,
    CompleteCoor, Game, Model, Time,
};
use log::Level;
use noise::{utils::NoiseMapBuilder, Seedable};
use poisson_diskus::bridson;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};
use voronoice::{BoundingBox, VoronoiBuilder};
use wasm_bindgen::prelude::*;

mod characters;

#[wasm_bindgen]
impl Model {
    pub fn create() -> Self {
        // let _creation_rng = StdRng::seed_from_u64(123);
        let mut creation_rng = StdRng::from_entropy();

        let mut universes = Universes::default();

        let (base_universe_id, base_universe) = universes.create(&mut creation_rng);

        let (base_planet_id, _base_planet) = base_universe.create_planet(&mut creation_rng);

        let graph_width = 10.;
        let graph_half_width = graph_width / 2.;

        let sites = {
            let box_size = [graph_width, graph_width];
            let rmin = graph_half_width / 10.;
            let k = 30;
            let use_pbc = false;

            bridson(&box_size, rmin, k, use_pbc)
                .expect("The map site wasn't generated")
                .into_iter()
                .map(|[x, y]| voronoice::Point {
                    x: x - graph_half_width,
                    y: y - graph_half_width,
                })
                .collect()
        };

        // builds a voronoi diagram from the set of sites above, bounded by a square of size 4
        let graph = VoronoiBuilder::default()
            .set_sites(sites)
            .set_bounding_box(BoundingBox::new_centered_square(graph_width))
            .set_lloyd_relaxation_iterations(2)
            .build()
            .unwrap();

        use noise::utils::PlaneMapBuilder;
        use noise::Fbm;

        let fbm = Fbm::new().set_seed(creation_rng.gen());

        let side = 15;
        let noise_model_ratio = side as f64 / graph_width;
        let noise_map = PlaneMapBuilder::new(&fbm)
            .set_size(side, side)
            .set_x_bounds(-graph_half_width / 2., graph_half_width / 2.)
            .set_y_bounds(-graph_half_width / 2., graph_half_width / 2.)
            .set_is_seamless(true)
            .build();

        let mut noise_map: Vec<_> = graph
            .iter_cells()
            .map(|cell| {
                let pos = cell.site_position();
                let (x, y) = (pos.x + graph_half_width, pos.y + graph_half_width);
                let (x, y) = (noise_model_ratio as f64 * x, noise_model_ratio as f64 * y);
                let noise = noise_map.get_value(x as usize, y as usize);

                noise
            })
            .collect();

        fn get_min_max(data: &Vec<f64>) -> (f64, f64) {
            if data.len() == 0 {
                unreachable!("should have at least 1 element in the noise vector");
            }
            let start = data[0];
            data.iter().fold((start, start), |(min, max), &cur| {
                if cur > max {
                    (min, cur)
                } else if cur < min {
                    (cur, max)
                } else {
                    (min, max)
                }
            })
        }

        let (min_noise, max_noise) = get_min_max(&noise_map);
        let normalize = |value: f64| value - min_noise / (max_noise - min_noise);

        noise_map
            .iter_mut()
            .for_each(|noise| *noise = normalize(*noise));

        let characters: Vec<_> = {
            let planet = universes.data[0].get_planet(base_planet_id);
            let land_region_indicies = planet.get_land_indices();

            let pre_build = vec![Character::new(
                "John".into(),
                CompleteCoor::OnPlanetRegion(
                    base_universe_id,
                    base_planet_id,
                    land_region_indicies[0].clone(),
                ),
            )]
            .into_iter();

            let random = (0..2000).map(|index| {
                Character::new(
                    format!("C{}", index),
                    CompleteCoor::OnPlanetRegion(
                        base_universe_id,
                        base_planet_id,
                        land_region_indicies
                            .choose(&mut creation_rng)
                            .expect("cannot randomly choose a land region")
                            .clone(),
                    ),
                )
            });

            pre_build.chain(random).collect()
        };

        Self {
            game: Game {
                tick: 0,
                characters,
                universes,
                player_characcter_id: 0,
                parties: Default::default(),
            },
        }
    }

    pub fn get_site_len(&self) -> usize {
        let universe = &self.game.universes.data[0];
        let planet = universe.get_planet_unsafe(0);
        planet.get_regions().len()
    }

    pub fn get_tick(&self) -> Time {
        self.game.tick
    }

    pub fn progress_game(&mut self) {
        self.game.progress()
    }

    pub fn create_planet_facade(&self) -> PlanetMapFacade {
        let universe = &self.game.universes.data[0];
        let planet = universe.get_planet_unsafe(0);

        PlanetMapFacade::from(planet)
    }

    pub fn count_universes(&self) -> usize {
        self.game.universes.data.len()
    }

    pub fn get_universe_name(&self, universe_id: usize) -> String {
        self.game.universes.data[universe_id].clone_name()
    }

    pub fn count_planets(&self, universe_id: usize) -> usize {
        self.game.universes.data[universe_id].count_planets()
    }

    pub fn get_planet_name(&self, universe_id: usize, planet_id: usize) -> String {
        self.game.universes.data[universe_id]
            .get_planet_unsafe(planet_id)
            .clone_name()
    }

    pub fn update_base_canvas_with_terrain(
        &self,
        universe_id: usize,
        planet_id: usize,
        grid_size: f64,
    ) {
        self.game.universes.data[universe_id]
            .get_planet_unsafe(planet_id)
            .update_base_canvas_with_terrain(grid_size);
    }

    pub fn update_base_canvas_with_height(
        &self,
        universe_id: usize,
        planet_id: usize,
        grid_size: f64,
    ) {
        self.game.universes.data[universe_id]
            .get_planet_unsafe(planet_id)
            .update_base_canvas_with_height(grid_size);
    }

    pub fn update_terrain_highlight_canvas(
        &self,
        universe_id: usize,
        planet_id: usize,
        grid_size: f64,
        region_id: usize,
    ) {
        self.game.universes.data[universe_id]
            .get_planet_unsafe(planet_id)
            .update_terrain_highlight_canvas(grid_size, region_id);
    }

    pub fn update_grid_canvas(&self, universe_id: usize, planet_id: usize, grid_size: f64) {
        self.game.universes.data[universe_id]
            .get_planet_unsafe(planet_id)
            .update_grid_canvas(grid_size);
    }

    pub fn find_region_index(&self, universe_id: usize, planet_id: usize, x: f64, y: f64) -> usize {
        let planet = self.game.universes.data[universe_id].get_planet_unsafe(planet_id);
        planet.find_region_index(PlanetCoor::new(x, y))
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Debug).expect("error initializing logger");
    console_error_panic_hook::set_once();
}
