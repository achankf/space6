use yew::prelude::*;

// use crate::character::character_view::CharacterView;

#[function_component(CharacterView)]
pub fn create_character_view() -> Html {
    /*
    let view_model = Rc::downgrade(&self.view_model);

    html! {
        <CharacterView view_model={view_model} />
    }
     */
    html! {
        <div>
            {"Character"}
        </div>
    }
}

/*
impl App {
    pub fn create_character_view(&self) -> Html {
        /*
        let view_model = Rc::downgrade(&self.view_model);

        html! {
            <CharacterView view_model={view_model} />
        }
         */
        html! {}
    }
}

 */
