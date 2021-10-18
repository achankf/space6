use std::borrow::Cow;

use yew::prelude::*;

use super::App;
use crate::{app::Msg, planet::PlanetId, universe::UniverseId};

impl App {
    pub fn create_planet_selector(&self) -> Html {
        let view_model = self.view_model.borrow();
        let link = &view_model.link;
        let game = &view_model.game;
        let (universe_id, planet_id) = view_model.get_selected_planet_id();

        let planet_selection = {
            let planet_id = planet_id.map(|planet_id| Cow::Owned(planet_id.to_string()));

            let on_change = link.callback(|e| {
                if let ChangeData::Select(select) = e {
                    let planet_id: usize = select.value().parse().unwrap();
                    Msg::UpdatePlanetId(PlanetId::new_unsafe(planet_id))
                } else {
                    panic!("not a selection");
                }
            });

            let planet_names = game
                .get_universe(universe_id)
                .get_planets()
                .iter()
                .enumerate()
                .map(|(index, planet)| {
                    let value = Cow::Owned(index.to_string());
                    let is_selected = view_model.is_planet_selected(PlanetId::new_unsafe(index));
                    html! {
                        <option value=&value selected=is_selected>{planet.clone_name()}</option>
                    }
                });

            html! {
                <select value=&planet_id onchange={on_change}>
                    {for planet_names}
                </select>
            }
        };

        let universe_names = game
            .get_universes()
            .iter()
            .enumerate()
            .map(|(index, universe)| {
                let value = Cow::Owned(index.to_string());
                html! {
                    <option value=&value>{universe.borrow_name()}</option>
                }
            });

        let universe_id: usize = universe_id.into();
        let universe_id = Cow::Owned(universe_id.to_string());

        let on_universe_change = link.callback(|e| {
            if let ChangeData::Select(select) = e {
                let universe_id: usize = select.value().parse().unwrap();
                Msg::UpdateUniverseId(UniverseId::new_unsafe(universe_id))
            } else {
                panic!("not a selection");
            }
        });

        html! {
            <>
                <select value=&universe_id onchange={on_universe_change}>
                    {for universe_names}
                </select>
                {planet_selection}
            </>
        }
    }
}
