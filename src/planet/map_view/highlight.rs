use super::{util::get_hover_canvas, PlanetMap};
use crate::planet::{map_view::util::get_context, Planet, RegionId};

impl PlanetMap {
    pub fn update_terrain_highlight_canvas(&self, planet: &Planet, region_id: RegionId) {
        let model_width = planet.model_width;
        let model_height = planet.model_height;
        let canvas = get_hover_canvas();
        let context = get_context(&canvas);

        let region = &planet.regions[usize::from(region_id)];

        context.save();

        self.set_transformation(&context);

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
}
