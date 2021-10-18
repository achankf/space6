use super::{
    util::{get_base_canvas, get_grid_canvas, get_hover_canvas, get_main_canvas},
    PlanetMap,
};
use crate::planet::map_view::util::get_context;

impl PlanetMap {
    pub fn redraw_main(&self) {
        let canvas = get_main_canvas();
        let context = get_context(&canvas);

        let hover_canvas = get_hover_canvas();
        let base_canvas = get_base_canvas();

        context
            .draw_image_with_html_canvas_element(&base_canvas, 0., 0.)
            .unwrap();

        if self.is_show_grid {
            let grid_canvas = get_grid_canvas();
            context
                .draw_image_with_html_canvas_element(&grid_canvas, 0., 0.)
                .unwrap();
        }

        context
            .draw_image_with_html_canvas_element(&hover_canvas, 0., 0.)
            .unwrap();
    }
}
