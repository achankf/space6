use std::collections::{BTreeSet, HashMap};

pub mod eid;
pub mod entities;
pub mod sequence;
pub mod tomb_vec;

/**
Stands for Entity Id generator (ids are redeemable).
*/
#[derive(Default)]
pub struct Eids<IndexT>
where
    IndexT: Ord,
{
    freed: BTreeSet<IndexT>,
    next: IndexT,
}

#[derive(Default)]
pub struct Sequence<IndexT> {
    next: IndexT,
}

/// inspired by https://github.com/fitzgen/generational-arena/blob/72975c8355949c2338976d944e047c9d9f447174/src/lib.rs#L178
/// but without the generation stuff.
enum Slot<DataT, IndexT> {
    Dead { next_free: IndexT },
    Alive(DataT),
}

/**
Short for tombstone-based vector. Features:
- index stability when deleting an element
- maintain freed list, and is basically free for large structs

Use cases: you have compact data that needs to be inserted & deleted while other objects maintain their index-based references.

Don't use it if:
- the data are sparse (use a HashMap instead)
- you don't need to remove data (use a Vec instead)
*/
pub struct Tec<DataT, IndexT = u32> {
    vec: Vec<Slot<DataT, IndexT>>,
    next_free: IndexT,
    count: usize,
}

/**
Entity container with the following features:
- stable indices and not redeemable
- generated indices
*/
pub struct Entities<DataT, IndexT = u32> {
    /// actual data
    data: HashMap<IndexT, DataT>,

    /// id generator
    next_id: IndexT,
}

/**
Successor trait for numbers.
*/
pub trait Succ {
    /// Return self + 1.
    fn succ(self) -> Self;
}

/**
A trait that describes the max value of an unsigned integer. This trait is used to detect overflow.
*/
pub trait Maximum {
    fn max_value() -> Self;
}

impl Succ for u8 {
    fn succ(self) -> Self {
        self + 1
    }
}

impl Maximum for u8 {
    fn max_value() -> Self {
        Self::MAX
    }
}

impl Succ for u16 {
    fn succ(self) -> Self {
        self + 1
    }
}

impl Maximum for u16 {
    fn max_value() -> Self {
        Self::MAX
    }
}

impl Succ for u32 {
    fn succ(self) -> Self {
        self + 1
    }
}

impl Maximum for u32 {
    fn max_value() -> Self {
        Self::MAX
    }
}

impl Succ for usize {
    fn succ(self) -> Self {
        self + 1
    }
}

impl Maximum for usize {
    fn max_value() -> Self {
        Self::MAX
    }
}

#[cfg(any(target_arch = "x86_32", target_arch = "x86_64", target_arch = "wasm32"))]
/**
Support usigned integer casting to usize, up to 32 bit (and usize itself), because the primary target is wasm32 at the time this was written.
*/
pub trait CastUsize {
    fn to(self) -> usize;
    fn from(val: usize) -> Self;
}

impl CastUsize for u8 {
    fn to(self) -> usize {
        self as usize
    }

    fn from(val: usize) -> Self {
        val as Self
    }
}

impl CastUsize for u16 {
    fn to(self) -> usize {
        self as usize
    }

    fn from(val: usize) -> Self {
        val as Self
    }
}

impl CastUsize for u32 {
    fn to(self) -> usize {
        self as usize
    }

    fn from(val: usize) -> Self {
        val as Self
    }
}

impl CastUsize for usize {
    fn to(self) -> usize {
        self
    }

    fn from(val: usize) -> Self {
        val
    }
}
