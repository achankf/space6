use std::collections::HashSet;

use crate::{character::CharacterId, entity::Tec};

enum AttackMove {
    Slash,
}

pub struct Unit {
    melee_attack: u32,
    melee_defense: u32,
    ranged_attack: u32,
    ranged_defense: u32,
    armor: u32, // hoi4-style armor: tank, barriers
    piercing: u32,
}

#[derive(Default)]
pub struct Party {
    characters: HashSet<CharacterId>,
}

#[derive(Default)]
pub struct Parties {
    free: Tec<Party>,
}
