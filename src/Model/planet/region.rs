use super::{Region, RegionId, RegionSpecialty};

impl Region {
    pub fn get_base_building_capacity(&self) -> u32 {
        10000
    }

    pub fn get_specialty(&self) -> RegionSpecialty {
        self.specialty
    }
}

impl From<usize> for RegionId {
    fn from(item: usize) -> RegionId {
        Self(item)
    }
}
