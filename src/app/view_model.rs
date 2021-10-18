use std::time::Duration;

use yew::{prelude::*, services::IntervalService};

use super::{App, MapSelection, Msg, View, ViewModel};
use crate::{
    planet::{PlanetId, RegionId},
    universe::UniverseId,
    Game,
};

impl ViewModel {
    pub fn new(game: Game, link: ComponentLink<App>) -> Self {
        Self {
            current_view: View::Map,
            game,
            game_loop_task: None,
            link,
            map_selection: MapSelection::Planet(UniverseId::new_unsafe(0), PlanetId::new_unsafe(0)),
        }
    }

    pub fn get_link(&self) -> &ComponentLink<App> {
        &self.link
    }

    pub fn try_select_planet(&mut self, planet_id: PlanetId) -> bool {
        match self.map_selection {
            MapSelection::Planet(universe_id, prev_planet_id, ..)
            | MapSelection::Region(universe_id, prev_planet_id, _) => {
                if prev_planet_id != planet_id {
                    self.map_selection = MapSelection::Planet(universe_id, planet_id);
                    true
                } else {
                    false
                }
            }
            MapSelection::Universe(universe_id) => {
                self.map_selection = MapSelection::Planet(universe_id, planet_id);
                true
            }
        }
    }

    pub fn try_select_universe(&mut self, universe_id: UniverseId) -> bool {
        match self.map_selection {
            MapSelection::Universe(prev_universe_id)
            | MapSelection::Planet(prev_universe_id, ..)
            | MapSelection::Region(prev_universe_id, _, _) => {
                if prev_universe_id != universe_id {
                    self.map_selection = MapSelection::Universe(universe_id);
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn try_select_region(&mut self, region_id: RegionId) -> bool {
        match self.map_selection {
            MapSelection::Universe(_) => {
                unreachable!("cannot select a region when the planet isn't selected")
            }
            MapSelection::Planet(prev_universe_id, prev_planet_id) => {
                self.map_selection =
                    MapSelection::Region(prev_universe_id, prev_planet_id, region_id);
                true
            }
            MapSelection::Region(prev_universe_id, prev_planet_id, prev_region_id) => {
                if prev_region_id != region_id {
                    self.map_selection =
                        MapSelection::Region(prev_universe_id, prev_planet_id, region_id);
                    log::info!("HIHIHIHI");
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn try_resume_game(&mut self) -> bool {
        assert!(
            !self.game_loop_task.is_some(),
            "cannot resume the game when it's already running"
        );
        let tick_per_second = 12;
        let frequency = Duration::from_millis(1000 / tick_per_second);

        let task = IntervalService::spawn(frequency, self.link.callback(|_| Msg::GameTick));
        self.game_loop_task = Some(task);
        true
    }

    pub fn try_pause_game(&mut self) -> bool {
        let has_task = self.game_loop_task.is_some();
        assert!(has_task, "cannot pause an already paused game");

        self.game_loop_task = None;

        has_task
    }

    pub fn is_planet_selected(&self, planet_id: PlanetId) -> bool {
        match &self.map_selection {
            MapSelection::Universe(_) => false,
            MapSelection::Planet(_, selected_planet_id)
            | MapSelection::Region(_, selected_planet_id, _) => *selected_planet_id == planet_id,
        }
    }

    pub fn progress_game_tick(&mut self) -> bool {
        self.game.progress();
        true
    }

    pub fn switch_view(&mut self, view: View) -> bool {
        if self.current_view != view {
            self.current_view = view;
            true
        } else {
            false
        }
    }

    pub fn get_game(&self) -> &Game {
        &self.game
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
