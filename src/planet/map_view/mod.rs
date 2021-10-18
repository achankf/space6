use std::{borrow::Cow, cell::RefCell, rc::Weak};

use nalgebra::Point2;
use rand_distr::num_traits::ToPrimitive;
use web_sys::MouseEvent;
use yew::{prelude::*, Properties};

use super::{Planet, RegionId};
use crate::{
    app::ViewModel,
    coor::CoorCalculator,
    util::{get_element::get_element_from_mouse_event, get_mouse_coor::get_mouse_coor},
};

mod grid_map;
mod height_map;
mod highlight;
mod main_canvas;
mod planet_map;
mod terrain_map;
mod util;

#[derive(PartialEq)]
pub enum MapMode {
    Terrain,
    Height,
}

pub struct PlanetMap {
    view_model: Weak<RefCell<ViewModel>>,
    link: ComponentLink<Self>,
    grid_size: f64,
    map_mode: MapMode,
    is_show_grid: bool,
    should_redraw: bool,
    hovered_region_id: Option<RegionId>,
}

pub enum Msg {
    MouseMove(MouseEvent),
    ChangeMapMode(MapMode),
    ToggleGrid,
}

#[derive(Clone, Properties)]
pub struct PlanetMapProps {
    pub view_model: Weak<RefCell<ViewModel>>,
    pub grid_size: f64,
}
