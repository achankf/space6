use std::rc::Rc;

use yew::prelude::*;

use super::App;
use crate::character::character_view::CharacterView;

impl App {
    pub fn create_character_view(&self) -> Html {
        let view_model = Rc::downgrade(&self.view_model);

        html! {
            <CharacterView view_model=view_model />
        }
    }
}
