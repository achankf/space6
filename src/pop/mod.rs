use std::collections::{HashMap, HashSet};

use enum_map::{Enum, EnumMap};

pub type PopCount = u32;

#[derive(Enum)]
pub enum Education {
    None,
    Basic,     // e.g. up to high school
    Good,      // e.g. undergrad to master
    Excellent, // e.g. doctor
}

pub enum Lifespan {
    Brief,  // bugs
    Short,  // dogs
    Normal, // human
    Long,   // trees
    Great,  // fantasy elfs
}

pub enum Traits {
    //
}

pub struct Race {
    lifespan: Lifespan,
    traits: HashSet<Traits>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct RaceId(usize);

#[derive(Enum)]
pub enum SocialClass {
    Low,
    Middle,
    Upper,
}

pub struct PopKey(RaceId, SocialClass, Education);

#[derive(Clone)]
pub struct PopData {
    number: PopCount,
    employed: PopCount,
}

#[derive(Default, Clone)]
pub struct Pops(HashMap<RaceId, EnumMap<SocialClass, EnumMap<Education, PopData>>>);

impl PopData {
    fn get_employed_count(&self) -> PopCount {
        self.number
    }

    fn get_unemployed_count(&self) -> PopCount {
        let Self { number, employed } = self;
        debug_assert!(number > employed);
        number - employed
    }
}

impl Pops {
    fn get(&self, PopKey(race_id, social_class, education): PopKey) -> Option<&PopData> {
        let by_race = self.0.get(&race_id);

        if let Some(class_map) = by_race {
            let ret = &class_map[social_class][education];
            Some(ret)
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        PopKey(race_id, social_class, education): PopKey,
    ) -> Option<&mut PopData> {
        let by_race = self.0.get_mut(&race_id);

        if let Some(class_map) = by_race {
            let ret = &mut class_map[social_class][education];
            Some(ret)
        } else {
            None
        }
    }
}
