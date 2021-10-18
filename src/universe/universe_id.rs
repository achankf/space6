use super::UniverseId;

impl UniverseId {
    pub fn new_unsafe(index: usize) -> Self {
        Self(index)
    }
}

impl From<UniverseId> for usize {
    fn from(id: UniverseId) -> Self {
        id.0
    }
}
