use std::{borrow::Cow, cell::RefCell, rc::Weak};

use nalgebra::Point2;
use rand_distr::num_traits::ToPrimitive;
use web_sys::MouseEvent;
use yew::{prelude::*, Properties};

use super::{Msg, Planet, PlanetMap, PlanetMapProps, RegionId};
use crate::{
    app::ViewModel,
    coor::CoorCalculator,
    planet::map_view::MapMode,
    util::{get_element::get_element_from_mouse_event, get_mouse_coor::get_mouse_coor},
};

impl Component for PlanetMap {
    type Message = Msg;
    type Properties = PlanetMapProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            view_model: props.view_model,
            link,
            grid_size: props.grid_size,
            map_mode: MapMode::Terrain,
            is_show_grid: false,
            should_redraw: true,
            hovered_region_id: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let view_model = self
            .view_model
            .upgrade()
            .expect("cannot upgrade 'view_model' to Rc");
        let mut view_model = view_model.borrow_mut();
        let game = view_model.get_game();
        let (universe_id, planet_id) = view_model.get_selected_planet_id();

        if let Some(planet_id) = planet_id {
            let universe = game.get_universe(universe_id);
            let planet = universe.get_planet(planet_id);

            match msg {
                Msg::MouseMove(e) => {
                    let model_height = planet.model_height;
                    let model_width = planet.model_width;
                    let coor_calculator =
                        CoorCalculator::new(model_width, model_height, self.grid_size);

                    let mouse_coor = get_mouse_coor(&e);
                    let model_coor = coor_calculator.to_model_coor(mouse_coor);
                    let region_id = planet.find_region_id(model_coor);

                    self.hovered_region_id = Some(region_id);
                    self.update_terrain_highlight_canvas(planet, region_id);
                    self.redraw_main();

                    true
                }
                Msg::ChangeMapMode(map_mode) => {
                    if self.map_mode != map_mode {
                        self.map_mode = map_mode;
                        self.update_base_canvas(planet);
                        self.redraw_main();
                    }

                    // update is handled by side-effects
                    false
                }
                Msg::ToggleGrid => {
                    self.is_show_grid = !self.is_show_grid;

                    if self.is_show_grid {
                        self.update_grid_canvas(planet);
                    }

                    self.redraw_main();

                    // update is handled by side-effects
                    false
                }
            }
        } else {
            false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.should_redraw = true;

        true
    }

    fn view(&self) -> Html {
        let view_model = self
            .view_model
            .upgrade()
            .expect("cannot upgrade 'view_model' to Rc");
        let view_model = view_model.borrow();
        let game = view_model.get_game();
        let (universe_id, planet_id) = view_model.get_selected_planet_id();

        if let Some(planet_id) = planet_id {
            let universe = game.get_universe(universe_id);
            let planet = universe.get_planet(planet_id);

            let model_height = planet.model_height;
            let model_width = planet.model_width;

            let coor_calculator = CoorCalculator::new(model_width, model_height, self.grid_size);

            // calculate the actual width & height of the canvas
            let vp_height = coor_calculator
                .to_vp_magnitude(model_height)
                .to_i16()
                .unwrap();
            let vp_width = coor_calculator
                .to_vp_magnitude(model_width)
                .to_i16()
                .unwrap();

            // turn the dimension values into strings
            let vp_height = Cow::Owned(vp_height.to_string());
            let vp_width = Cow::Owned(vp_width.to_string());

            let link = &self.link;

            let hovered_region_id = self.hovered_region_id;
            let update_global_selected_region_id = view_model.get_link().callback(move |_| {
                if let Some(region_id) = hovered_region_id {
                    crate::app::Msg::UpdateRegionId(region_id)
                } else {
                    unreachable!("clicked on the map without hovering a region")
                }
            });

            html! {
                <div>
                    <div>
                        <button onclick=link.callback(|_|
                            Msg::ChangeMapMode(MapMode::Terrain))>{"Terrain"}</button>
                        <button onclick=link.callback(|_|
                            Msg::ChangeMapMode(MapMode::Height))>{"Height"}</button>
                        <button onclick=link.callback(|_| Msg::ToggleGrid)>{"Grid"}</button>
                    </div>
                    <canvas id="map-hover" style="display: none" width=&vp_width height=&vp_height />
                    <canvas id="map-base" style="display: none" width=&vp_width height=&vp_height />
                    <canvas id="map-grid" style="display: none" width=&vp_width height=&vp_height />
                    <canvas id="map" style="cursor: none" width=&vp_width height=&vp_height
                        onmousemove=link.callback(|e| Msg::MouseMove(e))
                        onclick=update_global_selected_region_id />
                </div>
            }
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.should_redraw {
            self.redraw_all();
            self.should_redraw = false;
        }
    }
}

impl PlanetMap {
    fn redraw_all(&self) {
        let view_model = self
            .view_model
            .upgrade()
            .expect("cannot upgrade 'view_model' to Rc");
        let view_model = view_model.borrow();
        let game = view_model.get_game();
        let (universe_id, planet_id) = view_model.get_selected_planet_id();

        if let Some(planet_id) = planet_id {
            let universe = game.get_universe(universe_id);
            let planet = universe.get_planet(planet_id);

            self.update_base_canvas(planet);
            self.update_grid_canvas(planet);

            self.redraw_main();
        }
    }

    fn update_base_canvas(&self, planet: &Planet) {
        match self.map_mode {
            MapMode::Terrain => self.redraw_terrain(planet),
            MapMode::Height => self.update_base_canvas_with_height(planet),
        }
    }

    pub fn set_transformation(&self, context: &web_sys::CanvasRenderingContext2d) {
        let grid_size = self.grid_size;
        context.scale(grid_size, grid_size).unwrap();
        context.set_line_width(1. / grid_size)
    }
}
