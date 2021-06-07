use super::UniverseId;

impl From<usize> for UniverseId {
    fn from(item: usize) -> Self {
        UniverseId(item)
    }
}
