use wasm_bindgen::JsCast;

type Canvas = web_sys::HtmlCanvasElement;

pub fn get_canvas(element_id: &str) -> Canvas {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(element_id)
        .unwrap_or_else(|| panic!("cannot find canvas, id={}", element_id));
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn get_main_canvas() -> Canvas {
    get_canvas("map")
}

pub fn get_hover_canvas() -> Canvas {
    get_canvas("map-hover")
}

pub fn get_grid_canvas() -> Canvas {
    get_canvas("map-grid")
}

pub fn get_base_canvas() -> Canvas {
    get_canvas("map-base")
}

pub fn get_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn set_transformation(context: &web_sys::CanvasRenderingContext2d, grid_size: f64) {
    context.scale(grid_size, grid_size).unwrap();
    context.set_line_width(1. / grid_size)
}
