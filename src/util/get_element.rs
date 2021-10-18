use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

pub fn get_element_from_mouse_event(e: &MouseEvent) -> HtmlElement {
    e.target()
        .expect("mouse event doesn't have a target")
        .dyn_into::<HtmlElement>()
        .expect("event target should be of type HtmlElement")
}
