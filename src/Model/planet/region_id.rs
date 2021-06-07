use super::RegionId;

impl RegionId {
    pub fn new_unsafe(index: usize) -> Self {
        Self(index)
    }
}

impl From<RegionId> for usize {
    fn from(item: RegionId) -> Self {
        item.0
    }
}
