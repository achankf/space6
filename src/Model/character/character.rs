use super::Character;
use crate::CompleteCoor;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CharacterFacade {
    name: String,
    coor: CompleteCoor,
}

impl From<Character> for CharacterFacade {
    fn from(character: Character) -> Self {
        Self {
            name: character.name,
            coor: character.coor,
        }
    }
}

impl Character {
    pub fn copy_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_coor(&self) -> CompleteCoor {
        self.coor
    }
}
