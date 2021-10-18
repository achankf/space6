use nalgebra::Point2;
use yew::prelude::*;

use super::get_element::get_element_from_mouse_event;

pub fn get_mouse_coor(e: &MouseEvent) -> Point2<f64> {
    let rect = get_element_from_mouse_event(&e).get_bounding_client_rect();

    let x = e.client_x() as f64 - rect.left();
    let y = e.client_y() as f64 - rect.top();

    Point2::new(x, y)
}
