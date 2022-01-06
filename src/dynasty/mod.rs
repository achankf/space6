use std::collections::HashSet;

use crate::character::CharacterId;

pub mod dynasty;
mod dynasty_id;

pub struct Dynasty {
    pub head: CharacterId,
    pub members: HashSet<CharacterId>, // dies out if no member remains
}

#[derive(Clone, Copy)]
pub struct DynastyId(usize);
