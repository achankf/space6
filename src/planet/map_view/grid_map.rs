use wasm_bindgen::JsValue;

use super::{util::get_grid_canvas, PlanetMap};
use crate::planet::{map_view::util::get_context, Planet};

impl PlanetMap {
    pub fn update_grid_canvas(&self, planet: &Planet) {
        let context = {
            let canvas = get_grid_canvas();
            get_context(&canvas)
        };

        let style: JsValue = "gray".into();

        context.save();

        context.set_stroke_style(&style);

        self.set_transformation(&context);

        for region in &planet.regions {
            context.begin_path();

            let bvs = &region.border_vertices;
            let first = bvs[0];

            context.move_to(first.x, first.y);
            bvs.iter().skip(1).for_each(|coor| {
                context.line_to(coor.x, coor.y);
            });
            context.close_path();
            context.stroke();
        }
        context.restore();
    }
}
