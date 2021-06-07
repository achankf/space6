use crate::coor::CoorCalculator;
use crate::planet::canvas_wasm::util::{get_canvas, get_context};
use crate::planet::Planet;
use crate::terrain::Terrain;
use std::f64;
use wasm_bindgen::JsValue;

impl Planet {
    pub fn update_base_canvas_with_terrain(&self, grid_size: f64) {
        let Self {
            model_height,
            model_width,
            ..
        } = self;
        let model_width = *model_width;
        let model_height = *model_height;
        let coor_calculator = CoorCalculator::new(model_width, model_height, grid_size);

        let context = {
            let canvas = get_canvas("map-base");
            get_context(&canvas)
        };

        let hill_color: JsValue = "#654321".into();
        let mountain_color: JsValue = "#595959".into();
        let plain_color: JsValue = "green".into();
        let shallow_water_color: JsValue = "#0099FF".into();
        let deep_ocean_color: JsValue = "#0066FF".into();

        let map_background_color = |terrain: Terrain| -> &JsValue {
            match terrain {
                Terrain::Hill => &hill_color,
                Terrain::Mountain => &mountain_color,
                Terrain::Plain => &plain_color,
                Terrain::ShallowWater => &shallow_water_color,
                Terrain::DeepOcean => &deep_ocean_color,
            }
        };

        context.save();
        for region in &self.regions {
            let terrain = region.terrain;
            let style = map_background_color(terrain);

            context.begin_path();
            context.set_stroke_style(style);
            context.set_fill_style(style);

            let bvs = &region.border_vertices;
            let first = coor_calculator.to_vp_coor(bvs[0]);

            context.move_to(first.x, first.y);
            bvs.iter().skip(1).for_each(|coor| {
                let coor = coor_calculator.to_vp_coor(*coor);
                context.line_to(coor.x, coor.y);
            });
            context.close_path();
            context.fill();
            context.stroke();
        }
        context.restore();
    }
}
