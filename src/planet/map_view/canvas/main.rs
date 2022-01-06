use crate::{app::ViewModelContext, planet::map_view::MapMode};

use super::{
    base::update_base_canvas,
    grid::update_grid_canvas,
    util::{get_base_canvas, get_context, get_grid_canvas, get_hover_canvas, get_main_canvas},
};

pub fn redraw_main(is_show_grid: bool) {
    let canvas = get_main_canvas();
    let context = get_context(&canvas);

    let hover_canvas = get_hover_canvas();
    let base_canvas = get_base_canvas();

    context
        .draw_image_with_html_canvas_element(&base_canvas, 0., 0.)
        .unwrap();

    if is_show_grid {
        let grid_canvas = get_grid_canvas();
        context
            .draw_image_with_html_canvas_element(&grid_canvas, 0., 0.)
            .unwrap();
    }

    context
        .draw_image_with_html_canvas_element(&hover_canvas, 0., 0.)
        .unwrap();
}

pub fn redraw_all(
    view_model: &ViewModelContext,
    map_mode: MapMode,
    is_show_grid: bool,
    should_redraw_map: bool,
) {
    let game = view_model.game.borrow();
    let (universe_id, planet_id) = view_model.get_selected_planet_id();

    if let Some(planet_id) = planet_id {
        let universe = game.get_universe(universe_id);
        let planet = universe.get_planet(planet_id);

        if should_redraw_map {
            update_base_canvas(view_model, map_mode, planet);
        }
        update_grid_canvas(view_model, planet);

        redraw_main(is_show_grid);
    }
}
