use super::RegionId;

mod canvas;
pub(crate) mod planet_map;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapMode {
    Terrain,
    Height,
}
