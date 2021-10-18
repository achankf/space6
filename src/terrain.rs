use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Terrain {
    Mountain,
    Hill,
    Plain,
    ShallowWater,
    DeepOcean,
}

impl Terrain {
    pub fn is_land(&self) -> bool {
        match self {
            Terrain::Hill | Terrain::Plain | Terrain::Mountain => true,
            Terrain::DeepOcean | Terrain::ShallowWater => false,
        }
    }
}
