use std::collections::{HashMap, HashSet};

use crate::CompleteCoor;

mod character;
mod character_id;
pub mod character_view;

// Each character has a set of fixed random attributes
pub struct Potential {
    con: u8, // constitution
    int: u8, // intelligence
    cha: u8, // charisma
    bea: u8, // how beautiful the character is
    man: u8, // mana affinity
    luc: u8, // luck
    cre: u8, // creativity
    rfx: u8, // reflex
}

#[derive(Default)]
pub struct Experience {
    learning: u32,
    driving: u32,    // the ability to drive any vehicles
    management: u32, // how well a character can run an organization
    labor: u32,      // manual labor
    creation: u32,   // creating things or contents
    warfare: u32,
    leadership: u32,
    persuasion: u32,
    social: u32,
    adventuring: u32,
    cunning: u32,
}

#[derive(Default)]
pub struct Skill {
    math: u8,
    science: u8,
    alchemy: u8,
    martial_arts: u8,
    elementalist: u8,
}

#[derive(Default)]
pub struct Wellbeing {
    food: u32,
    happiness: u32,
    stress: u32,
    health: u32,
    sleep: u32,
}

#[derive(Clone, Copy)]
pub enum Parents {
    Two(CharacterId, CharacterId),
    One(CharacterId),
    Unknown,
}

#[derive(Clone)]
pub struct Character {
    name: String,
    coor: CompleteCoor,
    parents: Parents,
    children: HashSet<CharacterId>,
    character_opinion: HashMap<CharacterId, i8>,
}

pub struct HumanBody {
    head: u32,
    body: u32,
    left_hand: u32,
    right_hand: u32,
    left_foot: u32,
    right_foot: u32,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct CharacterId(usize);
