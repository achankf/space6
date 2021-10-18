use std::f64;

use wasm_bindgen::JsValue;

use super::{util::get_base_canvas, PlanetMap};
use crate::planet::{map_view::util::get_context, Planet};

impl PlanetMap {
    pub fn update_base_canvas_with_height(&self, planet: &Planet) {
        let context = {
            let canvas = get_base_canvas();
            get_context(&canvas)
        };

        context.save();

        self.set_transformation(&context);

        for region in &planet.regions {
            let style = to_gray(map_byte(region.noise));
            let style: JsValue = style.into();

            context.begin_path();
            context.set_stroke_style(&style);
            context.set_fill_style(&style);

            let bvs = &region.border_vertices;
            let first = bvs[0];

            context.move_to(first.x, first.y);
            bvs.iter().skip(1).for_each(|coor| {
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
