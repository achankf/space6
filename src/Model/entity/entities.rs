use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{Index, IndexMut},
};

use super::{CastUsize, Entities, Succ, Tec};

impl<DataT, IndexT> Entities<DataT, IndexT>
where
    IndexT: Succ + Clone + Copy + Default + Hash + Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
            ..Self::default()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: IndexT) -> Option<&DataT> {
        self.data.get(&index)
    }

    pub fn get_mut(&mut self, index: IndexT) -> Option<&mut DataT> {
        self.data.get_mut(&index)
    }

    /** Panic if index is invalid */
    pub fn remove(&mut self, index: IndexT) -> Option<DataT> {
        self.data.remove(&index)
    }

    pub fn alloc(&mut self, data: DataT) -> IndexT {
        let next_id = self.next_id;
        self.next_id = self.next_id.succ();
        self.data.insert(next_id, data);

        next_id
    }

    pub fn iter(&self) -> impl Iterator<Item = (IndexT, &DataT)> {
        self.data
            .iter()
            .map(|(virtual_id, data)| (*virtual_id, data))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (IndexT, &mut DataT)> {
        self.data
            .iter_mut()
            .map(|(virtual_id, data)| (*virtual_id, data))
    }
}

impl<DataT, IndexT> Default for Entities<DataT, IndexT>
where
    IndexT: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
            next_id: Default::default(),
        }
    }
}

impl<DataT, IndexT> Index<IndexT> for Entities<DataT, IndexT>
where
    IndexT: Default + Succ + Clone + Copy + Hash + Eq,
{
    type Output = DataT;

    fn index(&self, index: IndexT) -> &Self::Output {
        self.get(index).expect("element not exist")
    }
}

impl<DataT, IndexT> IndexMut<IndexT> for Entities<DataT, IndexT>
where
    IndexT: Default + Succ + Clone + Copy + Hash + Eq,
{
    fn index_mut(&mut self, index: IndexT) -> &mut Self::Output {
        self.get_mut(index).expect("element not exist")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::entity::Entities;

    #[test]
    #[should_panic(expected = "element not exist")]
    fn access_out_of_bound() {
        let mut entities = Entities::default();
        entities.alloc(1232);
        entities[312u16];
    }

    #[test]
    #[should_panic(expected = "element not exist")]
    fn access_out_of_bound_mut() {
        let mut entities = Entities::default();
        entities.alloc(1232);
        entities[312u16] = 3333;
    }

    #[test]
    fn normal() {
        let mut entities = Entities::new();

        let map = vec!["1", "2", "3", "4", "5"]
            .iter()
            .fold(HashMap::new(), |mut acc, data| {
                acc.insert(entities.alloc(*data), *data);
                acc
            });

        fn check_all(entities: &Entities<&str>) {
            entities
                .iter()
                .for_each(|(id, data)| assert_eq!(entities[id], *data));
        }

        map.iter()
            .for_each(|(id, data)| assert_eq!(entities[*id], *data));

        entities.remove(1);
        check_all(&entities);

        entities.remove(4);
        check_all(&entities);

        // after coalescing

        entities.remove(3);
        check_all(&entities);

        entities.remove(2);
        assert_eq!(entities.len(), 1);
        check_all(&entities);

        entities.remove(0);
        assert!(entities.is_empty());
        check_all(&entities);
    }
}
