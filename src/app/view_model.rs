use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use super::{Action, MapSelection, Model, View};
use crate::{
    planet::{PlanetId, RegionId},
    universe::UniverseId,
    Game,
};

impl Default for Model {
    fn default() -> Self {
        let game = Game::create();

        Self {
            current_view: View::Map,
            game: Rc::new(RefCell::new(game)),
            map_selection: MapSelection::Planet(UniverseId::new_unsafe(0), PlanetId::new_unsafe(0)),
            should_game_loop_run: false,
            grid_size: 15.,
            should_redraw_map: Rc::new(RefCell::new(true)),
        }
    }
}

impl Reducible for Model {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();

        match action {
            Action::UpdatePlanetId(id) => next.try_select_planet(id),
            Action::UpdateUniverseId(id) => next.try_select_universe(id),
            Action::UpdateRegionId(id) => next.try_select_region(id),
            Action::SwitchView(view) => next.switch_view(view),
            Action::PauseGame => next.try_pause_game(),
            Action::ResumeGame => next.try_resume_game(),
        };

        next.into()
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.current_view == other.current_view
            && self.map_selection == other.map_selection
            && self.should_game_loop_run == other.should_game_loop_run
            && self.game.borrow().generation == other.game.borrow().generation
    }
}

impl Model {
    pub fn try_select_planet(&mut self, planet_id: PlanetId) {
        match self.map_selection {
            MapSelection::Region(universe_id, _, _) => {
                self.map_selection = MapSelection::Planet(universe_id, planet_id);
            }
            MapSelection::Planet(universe_id, prev_planet_id, ..) => {
                if prev_planet_id != planet_id {
                    self.map_selection = MapSelection::Planet(universe_id, planet_id);
                }
            }
            MapSelection::Universe(universe_id) => {
                self.map_selection = MapSelection::Planet(universe_id, planet_id);
            }
        };
        *self.should_redraw_map.borrow_mut() = true;
    }

    pub fn try_select_universe(&mut self, universe_id: UniverseId) {
        match self.map_selection {
            MapSelection::Universe(prev_universe_id)
            | MapSelection::Planet(prev_universe_id, ..)
            | MapSelection::Region(prev_universe_id, _, _) => {
                if prev_universe_id != universe_id {
                    self.map_selection = MapSelection::Universe(universe_id);
                }
            }
        }
    }

    pub fn try_select_region(&mut self, region_id: RegionId) {
        match self.map_selection {
            MapSelection::Universe(_) => {
                unreachable!("cannot select a region when the planet isn't selected")
            }
            MapSelection::Planet(prev_universe_id, prev_planet_id) => {
                self.map_selection =
                    MapSelection::Region(prev_universe_id, prev_planet_id, region_id);
            }
            MapSelection::Region(prev_universe_id, prev_planet_id, prev_region_id) => {
                if prev_region_id != region_id {
                    self.map_selection =
                        MapSelection::Region(prev_universe_id, prev_planet_id, region_id);
                }
            }
        };
    }

    pub fn try_resume_game(&mut self) {
        self.should_game_loop_run = true;
    }

    pub fn try_pause_game(&mut self) {
        self.should_game_loop_run = false;
    }

    pub fn is_planet_selected(&self, planet_id: PlanetId) -> bool {
        match &self.map_selection {
            MapSelection::Universe(_) => false,
            MapSelection::Planet(_, selected_planet_id)
            | MapSelection::Region(_, selected_planet_id, _) => *selected_planet_id == planet_id,
        }
    }

    pub fn switch_view(&mut self, view: View) {
        if self.current_view != view {
            self.current_view = view;
        }
    }

    pub fn get_selected_planet_id(&self) -> (UniverseId, Option<PlanetId>) {
        match &self.map_selection {
            MapSelection::Universe(universe_id) => (*universe_id, None),
            MapSelection::Planet(universe_id, planet_id)
            | MapSelection::Region(universe_id, planet_id, _) => (*universe_id, Some(*planet_id)),
        }
    }

    pub fn get_selected_universe_id(&self) -> UniverseId {
        match &self.map_selection {
            MapSelection::Universe(universe_id)
            | MapSelection::Planet(universe_id, _)
            | MapSelection::Region(universe_id, _, _) => *universe_id,
        }
    }
}
