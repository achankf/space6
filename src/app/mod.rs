use std::{cell::RefCell, rc::Rc};

use crate::{
    planet::{PlanetId, RegionId},
    universe::UniverseId,
    Game,
};

mod character_view;
pub(crate) mod main;
mod map_view;
mod planet_selector;
mod view_model;

#[derive(PartialEq, Clone, Debug)]
pub enum View {
    Player,
    Map,
}

pub struct App {
    view_model: Rc<RefCell<Model>>,
}

#[derive(Debug)]
pub enum Action {
    UpdateUniverseId(UniverseId),
    UpdatePlanetId(PlanetId),
    UpdateRegionId(RegionId),
    SwitchView(View),
    ResumeGame,
    PauseGame,
}

#[derive(Clone)]
pub struct Model {
    pub current_view: View,
    pub game: Rc<RefCell<Game>>, // using Rc to avoid deep cloning of the (supposedly big) game object
    pub should_game_loop_run: bool,
    pub should_redraw_map: Rc<RefCell<bool>>,
    pub map_selection: MapSelection,
    pub grid_size: f64,
}

pub type ViewModelContext = yew::UseReducerHandle<Model>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapSelection {
    Universe(UniverseId),
    Planet(UniverseId, PlanetId),
    Region(UniverseId, PlanetId, RegionId),
}
