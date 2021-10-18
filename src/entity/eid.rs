use std::mem;

use super::{Eids, Maximum, Succ};

impl<IndexT> Eids<IndexT>
where
    IndexT: Succ + Clone + Copy + Ord + Default + Maximum,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn claim(&mut self) -> IndexT {
        self.freed
            .iter()
            .next()
            .cloned() //TODO replace with pop_first()
            .and_then(|x| {
                // found an id in the free list, return it
                let is_removed = self.freed.remove(&x);
                assert!(is_removed, "freeing something not in the database");
                Some(x)
            })
            .unwrap_or_else(|| {
                // otherwise increment the id and return it
                let next = self.next.succ();
                assert!(
                    next < IndexT::max_value(),
                    "storing more items that you can address"
                );
                mem::replace(&mut self.next, next)
            })
    }

    pub fn unclaim(&mut self, val: IndexT) {
        assert!(val < self.next, "not a valid entity");

        let is_double_inserted = self.freed.insert(val);
        assert!(is_double_inserted, "double-freeing entity")
    }
}

#[cfg(test)]
mod tests {
    use super::Eids;

    #[test]
    fn claim_ids() {
        let mut entities = Eids::new();
        for i in 0..100 {
            let id: u8 = entities.claim();
            assert_eq!(id, i);
        }

        fn is_multiple_of_3(i: &u8) -> bool {
            i % 3 == 0
        }

        (0..60u8)
            .filter(is_multiple_of_3)
            .for_each(|i| entities.unclaim(i));

        (0..60u8)
            .filter(is_multiple_of_3)
            .all(|i| entities.claim() == i);
    }

    #[test]
    #[should_panic(expected = "not a valid entity")]
    fn unclaim_invalid() {
        let mut entities = Eids::new();
        entities.unclaim(123u8)
    }

    #[test]
    #[should_panic(expected = "double-freed")]
    fn double_free() {
        let mut entities = Eids::new();
        let id: u8 = entities.claim();
        entities.unclaim(id);
        entities.unclaim(id);
    }

    #[test]
    #[should_panic(expected = "assertion failed: next < IndexT::max_value()")]
    fn claim_over_max() {
        let mut entities = Eids::<u8>::new();
        (0..257).for_each(|_| {
            entities.claim();
        });
    }
}
