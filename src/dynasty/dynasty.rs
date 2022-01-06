use std::collections::HashSet;

use super::{Dynasty, DynastyId};
use crate::{
    character::{Character, CharacterId},
    id::Id,
    CompleteCoor, Game,
};

pub struct NewCharacter {
    pub name: String,
    pub coor: CompleteCoor,
}

impl Game {
    pub fn start_dynasty(
        dynasties: &mut Vec<Dynasty>,
        characters: &mut Vec<Character>,
        NewCharacter { name, coor }: NewCharacter,
    ) -> (DynastyId, CharacterId) {
        let dynasty_id = DynastyId::next(dynasties);
        let character_id = CharacterId::next(characters);

        let character = Character {
            name,
            coor,
            dynasty_id,
            parents: Default::default(),
            children: Default::default(),
            relationships: Default::default(),
        };

        characters.push(character);

        let dynasty = Dynasty {
            head: character_id,
            members: HashSet::from([character_id]),
        };

        dynasties.push(dynasty);

        (dynasty_id, character_id)
    }

    pub fn birth(
        dynasties: &mut Vec<Dynasty>,
        characters: &mut Vec<Character>,
        NewCharacter { name, coor }: NewCharacter,
        parents: HashSet<CharacterId>,
        dynasty_id: DynastyId,
    ) -> CharacterId {
        let character_id = CharacterId::next(characters);

        let character = Character {
            name,
            coor,
            dynasty_id,
            parents,
            children: Default::default(),
            relationships: Default::default(),
        };

        characters.push(character);

        let is_inserted = dynasty_id
            .project_mut(dynasties)
            .expect("Dynasty does not exist")
            .members
            .insert(character_id);
        assert!(is_inserted);

        character_id
    }
}
