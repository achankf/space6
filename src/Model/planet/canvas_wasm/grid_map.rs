use crate::coor::CoorCalculator;
use crate::planet::canvas_wasm::util::{get_canvas, get_context};
use crate::planet::Planet;
use std::f64;
use wasm_bindgen::JsValue;

impl Planet {
    pub fn update_grid_canvas(&self, grid_size: f64) {
        let Self {
            model_height,
            model_width,
            ..
        } = self;
        let model_width = *model_width;
        let model_height = *model_height;
        let coor_calculator = CoorCalculator::new(model_width, model_height, grid_size);

        let context = {
            let canvas = get_canvas("map-grid");
            get_context(&canvas)
        };

        let style: JsValue = "aquamarine".into();

        context.save();
        context.set_stroke_style(&style);

        for region in &self.regions {
            context.begin_path();

            let bvs = &region.border_vertices;
            let first = coor_calculator.to_vp_coor(bvs[0]);

            context.move_to(first.x, first.y);
            bvs.iter().skip(1).for_each(|coor| {
                let coor = coor_calculator.to_vp_coor(*coor);
                context.line_to(coor.x, coor.y);
            });
            context.close_path();
            context.stroke();
        }
        context.restore();
    }
}
