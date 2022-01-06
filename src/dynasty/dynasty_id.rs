use super::{Dynasty, DynastyId};

impl From<DynastyId> for usize {
    fn from(id: DynastyId) -> usize {
        id.0
    }
}

impl crate::id::Id for DynastyId {
    type Source = Dynasty;

    fn next(data: &[Self::Source]) -> Self {
        Self(data.len())
    }
}
