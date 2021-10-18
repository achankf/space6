use super::{Character, CharacterId, Parents};
use crate::CompleteCoor;

impl Character {
    pub fn new(name: String, coor: CompleteCoor, parents: Parents) -> Self {
        if let Parents::Two(p1, p2) = parents {
            assert!(p1 != p2, "the two parents are the same person");
        }

        Self {
            name,
            coor,
            parents,
            children: Default::default(),
            character_opinion: Default::default(),
        }
    }

    pub fn copy_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_coor(&self) -> CompleteCoor {
        self.coor
    }

    pub fn get_parents(&self) -> Vec<CharacterId> {
        match &self.parents {
            Parents::Unknown => Vec::new(),
            Parents::One(parent) => vec![*parent],
            Parents::Two(p1, p2) => vec![*p1, *p2],
        }
    }

    pub fn is_parent(&self, target: CharacterId) -> bool {
        match &self.parents {
            Parents::Unknown => false,
            Parents::One(parent) => target == *parent,
            Parents::Two(p1, p2) => target == *p1 || target == *p2,
        }
    }

    pub fn get_children(&self) -> impl Iterator<Item = &CharacterId> {
        self.children.iter()
    }

    pub fn is_child(&self, target: CharacterId) -> bool {
        self.children.contains(&target)
    }

    pub fn get_known_people(&self) -> impl Iterator<Item = (&CharacterId, &i8)> {
        self.character_opinion.iter().map(|data| data)
    }
}
