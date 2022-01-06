use wasm_bindgen::JsValue;

use crate::{app::ViewModelContext, planet::Planet};

use super::util::{get_context, get_grid_canvas, set_transformation};

pub fn update_grid_canvas(view_model: &ViewModelContext, planet: &Planet) {
    let grid_size = view_model.grid_size;
    let context = {
        let canvas = get_grid_canvas();
        get_context(&canvas)
    };

    let style: JsValue = "gray".into();

    context.save();

    context.set_stroke_style(&style);

    set_transformation(&context, grid_size);

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
