use crate::coor::CoorCalculator;
use crate::planet::canvas_wasm::util::{get_canvas, get_context};
use crate::planet::Planet;
use std::f64;

impl Planet {
    pub fn update_terrain_highlight_canvas(&self, grid_size: f64, region_id: usize) {
        let Self {
            model_height,
            model_width,
            ..
        } = self;
        let model_width = *model_width;
        let model_height = *model_height;
        let coor_calculator = CoorCalculator::new(model_width, model_height, grid_size);

        let canvas = get_canvas("map-hover");
        let context = get_context(&canvas);

        let region = &self.regions[region_id];

        let bvs = &region.border_vertices;
        let first = coor_calculator.to_vp_coor(bvs[0]);

        let vp_width = coor_calculator.to_vp_magnitude(model_width);
        let vp_height = coor_calculator.to_vp_magnitude(model_height);

        context.begin_path();
        context.clear_rect(0., 0., vp_width, vp_height);
        context.move_to(first.x, first.y);
        bvs.iter().skip(1).for_each(|coor| {
            let coor = coor_calculator.to_vp_coor(*coor);
            context.line_to(coor.x, coor.y);
        });
        context.close_path();
        context.stroke();
    }
}
