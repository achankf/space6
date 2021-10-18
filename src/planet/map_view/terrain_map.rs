use wasm_bindgen::JsValue;

use super::{
    util::{get_base_canvas, get_context},
    PlanetMap,
};
use crate::{planet::Planet, terrain::Terrain};

impl PlanetMap {
    pub fn redraw_terrain(&self, planet: &Planet) {
        let grid_size = self.grid_size;

        let context = {
            let canvas = get_base_canvas();
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
        context.scale(grid_size, grid_size).unwrap();
        context.set_line_width(1. / grid_size);

        for region in &planet.regions {
            let terrain = region.terrain;
            let style = map_background_color(terrain);

            context.begin_path();
            context.set_stroke_style(style);
            context.set_fill_style(style);

            let bvs = &region.border_vertices;
            let first = bvs[0];

            context.move_to(first.x, first.y);
            bvs.iter().skip(1).for_each(|coor| {
                let coor = coor;
                context.line_to(coor.x, coor.y);
            });
            context.close_path();
            context.fill();
            context.stroke();
        }
        context.restore();
    }
}
