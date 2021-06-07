use std::ops::AddAssign;

use nalgebra::Point2;

type Coor2 = Point2<f64>;

pub struct CoorCalculator {
    top_left: Coor2,
    grid_size: f64,
    model_width: f64,
    model_height: f64,
}

impl CoorCalculator {
    pub fn new(model_width: f64, model_height: f64, grid_size: f64) -> Self {
        Self {
            top_left: Coor2::origin(),
            grid_size,
            model_width,
            model_height,
        }
    }

    pub fn to_vp_magnitude(&self, val: f64) -> f64 {
        val * self.grid_size
    }

    pub fn to_vp_coor(&self, coor: Coor2) -> Coor2 {
        let Self {
            top_left,
            grid_size,
            ..
        } = self;
        (coor + top_left.coords) * grid_size.clone()
    }

    pub fn to_model_coor(&self, coor: Coor2) -> Coor2 {
        let Self {
            top_left,
            grid_size,
            ..
        } = self;

        coor / grid_size.clone() - top_left.coords
    }
}
