use std::hash::Hash;

use super::{Sequence, Succ};

impl<IndexT> Sequence<IndexT>
where
    IndexT: Succ + Clone + Copy + Hash + Eq,
{
    pub fn new(start: IndexT) -> Self {
        Self { next: start }
    }

    pub fn next(&mut self) -> IndexT {
        let ret = self.next;
        self.next = ret.succ();
        ret
    }
}
