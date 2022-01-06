use yew::prelude::*;

use super::ViewModelContext;
use crate::{
    app::{planet_selector::PlanetSelector, Action, MapSelection},
    planet::map_view::planet_map::PlanetMap,
};

#[function_component(MapView)]
pub fn create_map_view() -> Html {
    let view_model = use_context::<ViewModelContext>().expect("no view model context found");

    match view_model.map_selection {
        MapSelection::Universe(_universe_id) => {
            html! {
                <PlanetSelector />
            }
        }
        MapSelection::Planet(_universe_id, _planet_id) => {
            html! {
                <div>
                    <PlanetSelector />
                    <PlanetMap />
                </div>
            }
        }
        MapSelection::Region(_universe_id, planet_id, region_id) => {
            let region_id: usize = region_id.into();

            html! {
                <>
                    <PlanetSelector />
                    <div>
                        <button onclick={move |_| view_model.dispatch(Action::UpdatePlanetId(planet_id))}>{"Map"}</button>
                        <div>
                            {"Region "} {region_id}
                        </div>
                    </div>
                </>
            }
        }
    }
}
/*
impl App {
    pub fn create_map_view(&self, ctx: &Context<Self>) -> Html {
        let view_model = Rc::downgrade(&self.view_model);
        let planet_selector = self.create_planet_selector(ctx);

        match self.view_model.borrow().map_selection {
            MapSelection::Universe(universe_id) => {
                html! {
                    {planet_selector}
                }
            }
            MapSelection::Planet(universe_id, planet_id) => {
                html! {
                    <div>
                        {planet_selector}
                        <PlanetMap view_model={view_model} grid_size={15.} />
                    </div>
                }
            }
            MapSelection::Region(universe_id, planet_id, region_id) => {
                let region_id: usize = region_id.into();

                let link = ctx.link();

                html! {
                    <>
                        {planet_selector}
                        <div>
                            <button onclick={link.callback(move |_| Msg::UpdatePlanetId(planet_id))}>{"Map"}</button>
                            <div>
                                {"Region "} {region_id}
                            </div>
                        </div>
                    </>
                }
            }
        }
    }
}

 */
