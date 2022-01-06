use wasm_bindgen::JsValue;

use crate::{
    app::ViewModelContext,
    planet::{map_view::MapMode, Planet},
    terrain::Terrain,
};

use super::util::{get_base_canvas, get_context, set_transformation};

pub fn update_base_canvas_with_height(view_model: &ViewModelContext, planet: &Planet) {
    let grid_size = view_model.grid_size;

    let context = {
        let canvas = get_base_canvas();
        get_context(&canvas)
    };

    context.save();

    set_transformation(&context, grid_size);

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

pub fn update_base_canvas(view_model: &ViewModelContext, map_mode: MapMode, planet: &Planet) {
    match map_mode {
        MapMode::Terrain => update_base_canvas_with_terrain(view_model, planet),
        MapMode::Height => update_base_canvas_with_height(view_model, planet),
    }
}

pub fn update_base_canvas_with_terrain(view_model: &ViewModelContext, planet: &Planet) {
    let grid_size = view_model.grid_size;

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
    set_transformation(&context, grid_size);

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

    (value * 255.) as u8
}
