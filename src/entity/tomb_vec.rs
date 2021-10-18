use std::{
    mem,
    ops::{Index, IndexMut},
};

use IntoIterator;

use super::{CastUsize, Maximum, Slot, Succ, Tec};

impl<DataT, IndexT> Default for Tec<DataT, IndexT>
where
    IndexT: Default,
{
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            next_free: Default::default(),
            count: 0,
        }
    }
}

impl<DataT, IndexT> Tec<DataT, IndexT>
where
    IndexT: Default + CastUsize + Succ + Ord + Clone + Copy + Maximum,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            ..Self::default()
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.iter().count(), self.count);
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.vec.clear();
        self.next_free = Default::default();
        self.count = 0;
    }

    /**
    Note: can store at most IndexT::max_value() - 1 elements, because
    the next free node needs to be count + 1.
    */
    pub fn alloc(&mut self, data: DataT) -> IndexT {
        assert!(
            self.len() < IndexT::max_value().to(),
            "exceed storage limit"
        );

        let data = Slot::Alive(data);
        let next = self.next_free;
        let next_usize = next.to();

        self.count += 1;

        if next.to() as usize >= self.vec.len() {
            self.vec.push(data);
            let succ = next.succ();
            assert!(succ > next, "integer overflow");
            self.next_free = succ;
        } else {
            self.next_free = match self.vec[next_usize] {
                Slot::Alive(_) => unreachable!("next empty slot is in used"),
                Slot::Dead { next_free } => next_free,
            };
            self.vec[next_usize] = data;
        }
        next
    }

    /** Panic if index is invalid */
    pub fn remove(&mut self, index: IndexT) -> DataT {
        assert!(index < IndexT::max_value());

        let len = self.vec.len();

        if len == 0 {
            panic!("removing an item from an empty container");
        }

        let index_usize = index.to();

        match self.vec[index_usize] {
            Slot::Alive(_) => {
                let mut ret = Slot::Dead {
                    next_free: self.next_free,
                };
                mem::swap(&mut ret, &mut self.vec[index_usize]);
                self.next_free = index;
                self.count -= 1;

                match ret {
                    Slot::Alive(data) => data,
                    Slot::Dead { .. } => panic!("Cannot unwrap a dead item"),
                }
            }
            Slot::Dead { .. } => panic!("removing a dead item"),
        }
    }

    pub fn get(&self, index: IndexT) -> Option<&DataT> {
        assert!(index < IndexT::max_value());
        self.vec.get(index.to()).and_then(|slot| match slot {
            Slot::Alive(data) => Some(data),
            Slot::Dead { .. } => None,
        })
    }

    pub fn get_mut(&mut self, index: IndexT) -> Option<&mut DataT> {
        assert!(index < IndexT::max_value());
        self.vec.get_mut(index.to()).and_then(|slot| match slot {
            Slot::Alive(data) => Some(data),
            Slot::Dead { .. } => None,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (IndexT, &DataT)> {
        self.vec
            .iter()
            .enumerate()
            .filter_map(|(id, data)| match data {
                Slot::Alive(data) => Some((CastUsize::from(id), data)),
                Slot::Dead { .. } => None,
            })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (IndexT, &mut DataT)> {
        self.vec
            .iter_mut()
            .enumerate()
            .filter_map(|(id, data)| match data {
                Slot::Alive(data) => Some((CastUsize::from(id), data)),
                Slot::Dead { .. } => None,
            })
    }

    pub fn into_iter(self) -> impl Iterator<Item = (IndexT, DataT)> {
        self.vec
            .into_iter()
            .enumerate()
            .filter_map(|(id, data)| match data {
                Slot::Alive(data) => Some((CastUsize::from(id), data)),
                Slot::Dead { .. } => None,
            })
    }

    /// The ratio of how much living data vs all data. Use this to determine when to coalesce the data.
    pub fn utility_ratio(&self) -> f64 {
        let total = self.vec.len();
        if total == 0 {
            // assume empty to be fully utilized
            1.
        } else {
            let live = self.len();
            (live as f64) / (total as f64)
        }
    }

    /**
    Coalesce the data by removing the dead slots. Takes a predicate "f"
    that takes (the data, old id, new id).
    */
    pub fn coalesce<F>(&mut self, mut f: F)
    where
        F: FnMut(&DataT, IndexT, IndexT),
    {
        let mut old_id = Default::default();
        let mut new_id = Default::default();
        self.vec.retain(|item| {
            let is_retain = match item {
                Slot::Dead { .. } => false,
                Slot::Alive(data) => {
                    f(data, old_id, new_id);
                    new_id = new_id.succ();
                    true
                }
            };
            old_id = old_id.succ();
            return is_retain;
        })
    }
}

impl<DataT, IndexT> Index<IndexT> for Tec<DataT, IndexT>
where
    IndexT: Default + CastUsize + Ord + Succ + Clone + Copy + Maximum,
{
    type Output = DataT;

    fn index(&self, index: IndexT) -> &Self::Output {
        self.get(index).expect("element not exist")
    }
}

impl<DataT, IndexT> IndexMut<IndexT> for Tec<DataT, IndexT>
where
    IndexT: Default + CastUsize + Ord + Succ + Clone + Copy + Maximum,
{
    fn index_mut(&mut self, index: IndexT) -> &mut Self::Output {
        self.get_mut(index).expect("element not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::Tec;

    #[test]
    #[should_panic(expected = "removing an item from an empty container")]
    fn remove_unallocated_element() {
        let mut tec = Tec::<u8>::new();
        tec.remove(12321);
    }

    #[test]
    #[should_panic(expected = "removing an item from an empty container")]
    fn index_overflow() {
        let mut tec = Tec::<u8>::new();
        tec.remove(12321);
    }

    #[test]
    #[should_panic(expected = "removing a dead item")]
    fn remove_dead_element() {
        let mut tec = Tec::new();
        tec.alloc(12);
        let id: u32 = tec.alloc(23);
        tec.alloc(23);

        tec.remove(id);
        tec.remove(id);
    }

    #[test]
    #[should_panic(expected = "exceed storage limit")]
    fn alloc_over_max_capacity() {
        let mut tec = Tec::<u8, u8>::new();
        (0..=u8::MAX).for_each(|val| {
            tec.alloc(val);
        });
    }

    #[test]
    fn it_works() {
        let mut tec = Tec::with_capacity(2);
        assert_eq!(tec.len(), 0);

        let e1 = 1212;
        let i1: u16 = tec.alloc(e1);
        assert_eq!(tec.len(), 1);
        assert_eq!(tec[i1], e1);

        let e2 = 31232;
        let i2 = tec.alloc(e2);
        assert_eq!(tec.len(), 2);
        assert_eq!(tec[i2], e2);

        tec.clear();
        assert_eq!(tec.len(), 0);

        let e1 = 1212;
        let i1 = tec.alloc(e1);
        assert_eq!(tec.len(), 1);
        assert_eq!(tec[i1], e1);

        let e2 = 31232;
        let i2 = tec.alloc(e2);
        assert_eq!(tec.len(), 2);
        assert_eq!(tec[i2], e2);
    }

    #[test]
    fn insert() {
        // test collect() & get & index & count
        let a = 12312;
        let b = 654645;
        let c = 0;
        let d = 123;
        let mut tec = Tec::<_, u8>::new();
        let a_id = tec.alloc(a);
        let b_id = tec.alloc(b);
        let c_id = tec.alloc(c);
        let d_id = tec.alloc(d);

        assert_eq!(tec.get(a_id).cloned(), Some(a));
        assert_eq!(tec.get(b_id).cloned(), Some(b));
        assert_eq!(tec.get(c_id).cloned(), Some(c));
        assert_eq!(tec.get(d_id).cloned(), Some(d));
        assert_eq!(tec[a_id], a);
        assert_eq!(tec[b_id], b);
        assert_eq!(tec[c_id], c);
        assert_eq!(tec[d_id], d);
        assert_eq!(tec.len(), 4);

        // test alloc()
        let e = 43243;
        let e_index = tec.alloc(e);
        assert_eq!(tec.len(), 5);
        assert_eq!(e_index, 4);
        assert_eq!(tec[e_index], e);

        let e_index = tec.alloc(e);
        assert_eq!(tec.len(), 6);
        assert_eq!(e_index, 5);
        assert_eq!(tec[e_index], e);
    }

    #[test]
    fn remove() {
        let mut tec = Tec::<_, u8>::new();

        (0..100u8).for_each(|val| {
            tec.alloc(val);
        });

        assert_eq!(tec.len(), 100);

        tec.remove(90);
        assert_eq!(tec.len(), 99);
        assert!(tec
            .iter()
            .take(90)
            .enumerate()
            .all(|(index, (_, val))| index as u8 == *val));
        let temp: Vec<_> = tec.iter().skip(90).enumerate().collect();
        assert_eq!(temp.len(), 9);
        assert!(temp
            .iter()
            .all(|&(index, (_, val))| (index as u8) + 91 == *val));

        // insert at the dead slot
        let e1 = 123;
        let i1 = tec.alloc(e1);
        assert_eq!(i1, 90);
        assert_eq!(tec[i1], e1);
        assert_eq!(tec.len(), 100);

        // remove twice then insert
        tec.remove(20);
        tec.remove(32);
        assert_eq!(tec.len(), 98);

        let e2 = 124;
        let e3 = 125;
        let i2 = tec.alloc(e2);
        assert_eq!(tec.len(), 99);
        assert_eq!(tec[i2], e2);
        assert_eq!(i2, 32);

        let i3 = tec.alloc(e3);
        assert_eq!(tec.len(), 100);
        assert_eq!(tec[i3], e3);
        assert_eq!(i3, 20);
    }
}
