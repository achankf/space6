use std::{cell::RefCell, rc::Rc};

use yew::{prelude::*, services::interval::IntervalTask};

use crate::{
    planet::{PlanetId, RegionId},
    universe::UniverseId,
    Game,
};

mod app;
mod character_view;
mod map_view;
mod planet_selector;
mod view_model;

#[derive(PartialEq)]
pub enum View {
    Player,
    Map,
}

pub struct App {
    view_model: Rc<RefCell<ViewModel>>,
}

pub enum Msg {
    UpdateUniverseId(UniverseId),
    UpdatePlanetId(PlanetId),
    UpdateRegionId(RegionId),
    SwitchView(View),
    ResumeGame,
    PauseGame,
    GameTick,
}

pub struct ViewModel {
    current_view: View,
    game: Game,
    game_loop_task: Option<IntervalTask>,
    link: ComponentLink<App>,
    map_selection: MapSelection,
}

#[derive(Clone, Copy)]
pub enum MapSelection {
    Universe(UniverseId),
    Planet(UniverseId, PlanetId),
    Region(UniverseId, PlanetId, RegionId),
}
