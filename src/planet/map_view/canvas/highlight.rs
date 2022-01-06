use crate::planet::{Planet, RegionId};

use super::util::{get_context, get_hover_canvas, set_transformation};

pub fn update_highlight_canvas(planet: &Planet, region_id: RegionId, grid_size: f64) {
    let model_width = planet.model_width;
    let model_height = planet.model_height;
    let canvas = get_hover_canvas();
    let context = get_context(&canvas);

    let region = &planet.regions[usize::from(region_id)];

    context.save();

    set_transformation(&context, grid_size);

    // clear the previous highlight
    context.clear_rect(0., 0., model_width, model_height);

    context.begin_path();

    let bvs = &region.border_vertices;
    let first = bvs[0];

    context.move_to(first.x, first.y);
    bvs.iter().skip(1).for_each(|coor| {
        let coor = coor;
        context.line_to(coor.x, coor.y);
    });
    context.close_path();
    context.stroke();

    context.restore();
}
