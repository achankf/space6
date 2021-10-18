use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Seedable,
};
use ordered_float::OrderedFloat;
use poisson_diskus::bridson;
use rand::Rng;
use rstar::{PointDistance, RTree, RTreeObject, AABB};
use voronoice::{BoundingBox, VoronoiBuilder};

use super::{Planet, RegionId, RegionIndexData};
use crate::{
    planet::{PlanetCoor, Region, RegionSpecialty},
    terrain::Terrain,
    GalaxyEntityId,
};

impl RTreeObject for RegionIndexData {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.coor.x, self.coor.y])
    }
}

impl PointDistance for RegionIndexData {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        nalgebra::distance_squared(&self.coor, &PlanetCoor::new(point[0], point[1]))
    }
}

impl Planet {
    pub fn clone_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_regions(&self) -> &[Region] {
        &self.regions
    }

    pub fn new(creation_rng: &mut dyn rand_core::RngCore, id: GalaxyEntityId) -> Self {
        let model_width = creation_rng.gen_range(20.0..50.0);
        let model_height = model_width * 0.7;

        let sites: Vec<_> = {
            let box_size = [model_width, model_height];
            // let rmin = 1.;
            let rmin = 1.;
            let num_attempts = 30;
            let use_pbc = false;

            bridson(&box_size, rmin, num_attempts, use_pbc)
                .expect("The map site wasn't generated")
                .into_iter()
                .map(|[x, y]| voronoice::Point { x, y })
                .collect()
        };

        // builds a voronoi diagram from the set of sites above, bounded by a square of size 4
        let graph = VoronoiBuilder::default()
            .set_sites(sites)
            .set_bounding_box(BoundingBox::new(
                voronoice::Point {
                    x: model_width / 2.,
                    y: model_height / 2.,
                },
                model_width,
                model_height,
            ))
            .set_lloyd_relaxation_iterations(2)
            .build()
            .unwrap();

        let num_sites = graph.sites().len();

        // build the height map with library
        let fbm = Fbm::new().set_seed(creation_rng.gen());
        let base_noise_map = PlaneMapBuilder::new(&fbm)
            .set_size(model_width as usize, model_height as usize)
            .set_is_seamless(false)
            .build();

        // extract the library model
        let noise_map: Vec<_> = graph
            .iter_cells()
            .map(|cell| {
                let pos = cell.site_position();
                // map coordinates for model sized boudning rectangle to the noise one
                let noise = base_noise_map.get_value(pos.x as usize, pos.y as usize);

                noise
            })
            .collect();

        // normalize the noise to the 0-1 range
        let (min_noise, max_noise) = get_min_max(&noise_map);
        let normalize = |noise| crate::util::normalize::f64(noise, min_noise, max_noise);
        let noise_map: Vec<_> = noise_map.into_iter().map(normalize).collect();

        let noise_sorted = {
            let mut ret: Vec<OrderedFloat<f64>> =
                noise_map.iter().cloned().map(Into::into).collect();
            ret.sort();
            ret
        };

        let threshold = |proportion: f32| {
            let pos = (num_sites as f32 * proportion) as usize;
            noise_sorted[pos]
        };

        let noise_to_terrain = |value: f64| -> Terrain {
            {
                // https://sciencing.com/four-major-landforms-8205803.html
                // 50% plains
                // 45% hills
                // 5% mountains

                if value.le(&threshold(0.45)) {
                    Terrain::DeepOcean
                } else if value.le(&threshold(0.7)) {
                    Terrain::ShallowWater
                } else if value.le(&threshold(0.85)) {
                    Terrain::Plain
                } else if value.le(&threshold(0.95)) {
                    Terrain::Hill
                } else {
                    Terrain::Mountain
                }
            }
        };

        let regions: Vec<_> = noise_map
            .iter()
            .zip(graph.iter_cells())
            .map(|(&noise, cell)| {
                let centroid_coor = cell.site_position();
                let centroid_coor: PlanetCoor = PlanetCoor::new(centroid_coor.x, centroid_coor.y);

                let border_vertices = cell
                    .iter_vertices()
                    .cloned()
                    .into_iter()
                    .map(|point| PlanetCoor::new(point.x, point.y))
                    .collect();

                let neighbours = cell.iter_neighbors().map(RegionId::new_unsafe).collect();

                let terrain = noise_to_terrain(noise);

                Region {
                    centroid_coor,
                    border_vertices,
                    neighbours,
                    noise,
                    terrain,
                    specialty: RegionSpecialty::Uncivilized,
                    stability: 0,
                    local_market: Default::default(),
                    farms: Default::default(),
                    pops: Default::default(),
                }
            })
            .collect();

        let region_index = RTree::bulk_load(
            regions
                .iter()
                .enumerate()
                .map(|(index, region)| RegionIndexData {
                    index,
                    coor: region.centroid_coor,
                })
                .collect(),
        );

        let name = format!("Earth {}", creation_rng.next_u32()).to_owned();

        Self {
            id,
            name,
            regions,
            region_index,
            model_width,
            model_height,
        }
    }

    pub fn find_region_id(&self, coor: PlanetCoor) -> RegionId {
        let index = self
            .region_index
            .nearest_neighbor(&[coor.x, coor.y])
            .expect("should find at least 1 region")
            .index;
        RegionId::new_unsafe(index)
    }

    pub fn get_land_indices(&self) -> Vec<RegionId> {
        self.regions
            .iter()
            .enumerate()
            .filter(|(_, region)| region.terrain.is_land())
            .map(|(index, _)| RegionId::new_unsafe(index))
            .collect()
    }
}

fn get_min_max(data: &[f64]) -> (f64, f64) {
    if data.len() == 0 {
        unreachable!("noise array should have at least 1 value");
    }
    let first = data[0];

    data.iter().fold((first, first), |(min, max), &cur| {
        if cur > max {
            (min, cur)
        } else if cur < min {
            (cur, max)
        } else {
            (min, max)
        }
    })
}
