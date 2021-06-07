use super::PlanetId;

impl PlanetId {
    pub fn new_unsafe(index: usize) -> Self {
        Self(index)
    }
}

impl From<PlanetId> for usize {
    fn from(item: PlanetId) -> Self {
        item.0
    }
}
