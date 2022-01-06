use noise::{utils::NoiseMapBuilder, Seedable};
use poisson_diskus::bridson;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};
use voronoice::{BoundingBox, VoronoiBuilder};

use crate::{
    dynasty::dynasty::NewCharacter,
    universe::{Universe, UniverseId, Universes},
    CompleteCoor, Game, Time,
};

impl Game {
    pub fn create() -> Self {
        // let _creation_rng = StdRng::seed_from_u64(123);
        let mut creation_rng = StdRng::from_entropy();

        let mut universes = Universes::default();

        let (base_universe_id, base_universe) = universes.create(&mut creation_rng);

        let (base_planet_id, _base_planet) = base_universe.create_planet(&mut creation_rng);

        base_universe.create_planet(&mut creation_rng);
        base_universe.create_planet(&mut creation_rng);

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

        use noise::{utils::PlaneMapBuilder, Fbm};

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

        let (dynasties, characters): (Vec<_>, Vec<_>) = {
            let planet = universes.get_universes()[0].get_planet(base_planet_id);
            let land_region_indicies = planet.get_land_indices();

            let pre_build = vec![(
                "John".into(),
                CompleteCoor::OnPlanetRegion(
                    base_universe_id,
                    base_planet_id,
                    land_region_indicies[0].clone(),
                ),
            )];

            let random = (0..2000).map(|index| {
                let name = format!("C{}", index);
                let coor = CompleteCoor::OnPlanetRegion(
                    base_universe_id,
                    base_planet_id,
                    land_region_indicies
                        .choose(&mut creation_rng)
                        .expect("cannot randomly choose a land region")
                        .clone(),
                );
                (name, coor)
            });

            let total_len = random.len() + 1;

            pre_build.into_iter().chain(random).fold(
                (Vec::with_capacity(total_len), Vec::with_capacity(total_len)),
                |(mut dynasties, mut characters), (name, coor)| {
                    Self::start_dynasty(
                        &mut dynasties,
                        &mut characters,
                        NewCharacter { name, coor },
                    );
                    (dynasties, characters)
                },
            )
        };

        Self {
            characters,
            dynasties,
            generation: 0,
            parties: Default::default(),
            player_character_id: 0,
            tick: 0,
            universes,
        }
    }

    pub fn progress(&mut self) {
        self.tick += 1;
    }

    pub fn get_universes(&self) -> &[Universe] {
        &self.universes.get_universes()
    }

    pub fn get_universe(&self, universe_id: UniverseId) -> &Universe {
        &self.get_universes()[usize::from(universe_id)]
    }

    pub fn get_time(&self) -> Time {
        self.tick
    }
}
