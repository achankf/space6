use super::{Sequence, Succ};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{Index, IndexMut},
};

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
