use std::collections::HashSet;

use super::{Character, CharacterId, Relationship};
use crate::CompleteCoor;

impl Character {
    pub fn copy_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_coor(&self) -> CompleteCoor {
        self.coor
    }

    pub fn get_parents(&self) -> &HashSet<CharacterId> {
        &self.parents
    }

    pub fn is_parent(&self, target: CharacterId) -> bool {
        self.parents.contains(&target)
    }

    pub fn get_children(&self) -> impl Iterator<Item = &CharacterId> {
        self.children.iter()
    }

    pub fn is_child(&self, target: CharacterId) -> bool {
        self.children.contains(&target)
    }

    pub fn get_known_people(&self) -> impl Iterator<Item = (&CharacterId, &Relationship)> {
        self.relationships.iter().map(|data| data)
    }
}
