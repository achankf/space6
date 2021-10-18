use std::rc::Rc;

use yew::prelude::*;

use super::App;
use crate::{app::MapSelection, planet::map_view::PlanetMap};

impl App {
    pub fn create_map_view(&self) -> Html {
        let view_model = Rc::downgrade(&self.view_model);
        let planet_selector = self.create_planet_selector();

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
                        <PlanetMap view_model=view_model grid_size=15. />
                    </div>
                }
            }
            MapSelection::Region(universe_id, planet_id, region_id) => {
                let region_id: usize = region_id.into();

                html! {
                    <>
                        {planet_selector}
                        <div>
                            {"Region "} {region_id}
                        </div>
                    </>
                }
            }
        }
    }
}
