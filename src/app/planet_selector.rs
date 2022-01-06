use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    app::{Action, ViewModelContext},
    planet::PlanetId,
    universe::UniverseId,
};

#[function_component(PlanetSelector)]
pub fn create_planet_selector() -> Html {
    let view_model = use_context::<ViewModelContext>().expect("no view model context found");
    let game = view_model.game.borrow();
    let (universe_id, planet_id) = view_model.get_selected_planet_id();

    let planet_selection = {
        let planet_id = planet_id.map(|planet_id| planet_id.to_string());

        let onchange = {
            let view_model = view_model.clone();

            move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let planet_id: usize = input.value().parse().expect("cannot parse planet id");
                view_model.dispatch(Action::UpdatePlanetId(PlanetId::new_unsafe(planet_id)));
            }
        };

        let planet_names = game
            .get_universe(universe_id)
            .get_planets()
            .iter()
            .enumerate()
            .map(|(index, planet)| {
                let value = index.to_string();
                let is_selected = view_model.is_planet_selected(PlanetId::new_unsafe(index));
                html! {
                    <option value={value} selected={is_selected}>{planet.clone_name()}</option>
                }
            });

        html! {
            <select value={planet_id} {onchange} >
                {for planet_names}
            </select>
        }
    };

    let universe_selection = {
        let universe_names = game
            .get_universes()
            .iter()
            .enumerate()
            .map(|(index, universe)| {
                let value = index.to_string();
                html! {
                    <option value={value}>{universe.borrow_name()}</option>
                }
            });

        let onchange = {
            let view_model = view_model.clone();

            move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let universe_id: usize = input.value().parse().expect("cannot parse planet id");
                view_model.dispatch(Action::UpdateUniverseId(UniverseId::new_unsafe(
                    universe_id,
                )));
            }
        };

        let universe_id: usize = universe_id.into();

        html! {
            <select value={universe_id.to_string()} {onchange}>
                {for universe_names}
            </select>
        }
    };

    html! {
        <>
            {universe_selection}
            {planet_selection}
        </>
    }
}
