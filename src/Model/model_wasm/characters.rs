use crate::{
    character::{Character, CharacterId},
    planet::{planet_wasm::PlanetMapFacade, PlanetCoor},
    universe::Universes,
    CompleteCoor, Game, Model, Time,
};
use js_sys::Array;
use log::Level;
use noise::{utils::NoiseMapBuilder, Seedable};
use poisson_diskus::bridson;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};
use serde::{Deserialize, Serialize};
use voronoice::{BoundingBox, VoronoiBuilder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Serialize)]
pub struct CharacterWasm {
    pub id: usize,
    name: String,
    coor: CompleteCoor,
}

impl CharacterWasm {
    fn new(id: usize, character: Character) -> CharacterWasm {
        let name = character.copy_name();
        let coor = character.get_coor();

        CharacterWasm { id, name, coor }
    }
}

#[wasm_bindgen]
impl CharacterWasm {
    pub fn copy_name(&self) -> String {
        self.name.clone()
    }

    pub fn copy_coor(&self) -> JsValue {
        JsValue::from_serde(&self.coor).unwrap()
    }
}

#[wasm_bindgen]
pub struct CharacterListWasm {
    data: Vec<CharacterWasm>,
}

#[wasm_bindgen]
impl CharacterListWasm {
    pub fn at(&self, index: usize) -> CharacterWasm {
        self.data[index].clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[wasm_bindgen]
impl Model {
    pub fn get_all_characters(&self) -> CharacterListWasm {
        let data = self
            .game
            .characters
            .iter()
            .enumerate()
            .map(|(id, character)| CharacterWasm::new(id, character.clone()))
            .collect();

        CharacterListWasm { data }
    }

    pub fn get_all_characters2(&self) -> JsValue {
        let data: Vec<_> = self
            .game
            .characters
            .iter()
            .enumerate()
            .map(|(id, character)| CharacterWasm::new(id, character.clone()))
            .collect();

        JsValue::from_serde(&data).unwrap()
    }

    pub fn get_character(&self, player_id: usize) -> CharacterWasm {
        let character = self.game.characters[player_id].clone();
        CharacterWasm::new(player_id, character)
    }

    pub fn get_player_character_id(&self) -> usize {
        self.game.player_characcter_id
    }
}
