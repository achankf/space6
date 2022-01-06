use rand_distr::num_traits::ToPrimitive;

use yew::prelude::*;

use super::{canvas::main::redraw_all, RegionId};
use crate::{
    app::ViewModelContext,
    coor::CoorCalculator,
    planet::map_view::{
        canvas::{highlight::update_highlight_canvas, main::redraw_main},
        MapMode,
    },
    util::get_mouse_coor::get_mouse_coor,
};

#[function_component(PlanetMap)]
pub fn create_planet_map() -> Html {
    let view_model = use_context::<ViewModelContext>().expect("no view model context found");
    let should_redraw_map = view_model.should_redraw_map.clone();

    let map_mode = use_state(|| MapMode::Terrain);
    let is_show_grid = use_state(|| false);
    let hovered_region_id = use_state(|| Option::<RegionId>::None);

    let change_map_mode = {
        let map_mode = map_mode.clone();
        let should_redraw_map = should_redraw_map.clone();
        move |new_map_mode| {
            let map_mode = map_mode.clone();
            let should_redraw_map = should_redraw_map.clone();
            move |_| {
                if new_map_mode != *map_mode {
                    // note: order is important, mutable refs need to be updated before updating states, which triggers a rerender
                    *should_redraw_map.borrow_mut() = true;
                    map_mode.set(new_map_mode);
                }
            }
        }
    };

    let toggle_grid = {
        let is_show_grid = is_show_grid.clone();
        move |_| is_show_grid.set(!*is_show_grid)
    };

    let grid_size = view_model.grid_size;
    let game = view_model.game.borrow();
    let (universe_id, planet_id) = view_model.get_selected_planet_id();

    use_effect({
        let view_model = view_model.clone();
        let map_mode = *map_mode;
        let is_show_grid = *is_show_grid;

        move || {
            redraw_all(
                &view_model,
                map_mode,
                is_show_grid,
                *should_redraw_map.borrow(),
            );
            *should_redraw_map.borrow_mut() = false;

            || {}
        }
    });

    if let Some(planet_id) = planet_id {
        let universe = game.get_universe(universe_id);
        let planet = universe.get_planet(planet_id);

        let model_height = planet.model_height;
        let model_width = planet.model_width;

        let coor_calculator = CoorCalculator::new(model_width, model_height, grid_size);

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
        let vp_height = vp_height.to_string();
        let vp_width = vp_width.to_string();

        let update_selected_region_id = {
            let hovered_region_id = hovered_region_id.clone();
            let view_model = view_model.clone();

            move |_| {
                let region_id = hovered_region_id.expect("nothing is hovered");
                view_model.dispatch(crate::app::Action::UpdateRegionId(region_id));
            }
        };

        let map_mouse_move = {
            let is_show_grid = *is_show_grid;
            let view_model = view_model.clone();

            move |e| {
                let game = view_model.game.borrow();
                let universe = game.get_universe(universe_id);
                let planet = universe.get_planet(planet_id);
                let coor_calculator = CoorCalculator::new(model_width, model_height, grid_size);

                let mouse_coor = get_mouse_coor(&e);
                let model_coor = coor_calculator.to_model_coor(mouse_coor);
                let region_id = planet.find_region_id(model_coor);

                hovered_region_id.set(Some(region_id));
                update_highlight_canvas(planet, region_id, grid_size);
                redraw_main(is_show_grid);
            }
        };

        html! {
            <div>
                <div>
                    <button onclick={change_map_mode(MapMode::Terrain)}>{"Terrain"}</button>
                    <button onclick={change_map_mode(MapMode::Height)}>{"Height"}</button>
                    <button onclick={toggle_grid}>{"Grid"}</button>
                </div>
                <canvas id="map-hover" style="display: none" width={vp_width.clone()} height={vp_height.clone()} />
                <canvas id="map-base" style="display: none" width={vp_width.clone()} height={vp_height.clone()} />
                <canvas id="map-grid" style="display: none" width={vp_width.clone()} height={vp_height.clone()} />
                <canvas id="map" style="cursor: none" width={vp_width.clone()} height={vp_height.clone()}
                    onmousemove={map_mouse_move}
                    onclick={update_selected_region_id} />
            </div>
        }
    } else {
        html! {}
    }
}
