use crate::coor::CoorCalculator;
use crate::planet::canvas_wasm::util::{get_canvas, get_context};
use crate::planet::Planet;
use std::f64;
use wasm_bindgen::JsValue;

impl Planet {
    pub fn update_base_canvas_with_height(&self, grid_size: f64) {
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

        context.save();
        for region in &self.regions {
            let style = to_gray(map_byte(region.noise));
            let style: JsValue = style.into();

            context.begin_path();
            context.set_stroke_style(&style);
            context.set_fill_style(&style);

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

fn to_gray(byte: u8) -> String {
    format!("rgb({0},{0},{0})", byte)
}

fn map_byte(value: f64) -> u8 {
    if value > 1. {
        panic!("Value must be <= 1")
    };
    if value < 0. {
        panic!("Value must be >= 0")
    };

    return (value * 255.) as u8;
}
