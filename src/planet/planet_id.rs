use std::fmt;

use super::PlanetId;

impl PlanetId {
    pub fn new_unsafe(index: usize) -> Self {
        Self(index)
    }
}

impl From<PlanetId> for usize {
    fn from(id: PlanetId) -> Self {
        id.0
    }
}

impl fmt::Display for PlanetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
