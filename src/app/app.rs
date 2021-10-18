use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use super::{App, Msg, View, ViewModel};
use crate::Game;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let game = Game::create();
        let view_model = ViewModel::new(game, link);
        let view_model = Rc::new(RefCell::new(view_model));

        Self { view_model }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut view_model = self.view_model.borrow_mut();

        match msg {
            Msg::UpdatePlanetId(id) => view_model.try_select_planet(id),
            Msg::UpdateUniverseId(id) => view_model.try_select_universe(id),
            Msg::UpdateRegionId(id) => view_model.try_select_region(id),
            Msg::SwitchView(view) => view_model.switch_view(view),
            Msg::PauseGame => view_model.try_pause_game(),
            Msg::ResumeGame => view_model.try_resume_game(),
            Msg::GameTick => view_model.progress_game_tick(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view_model = self.view_model.borrow();

        let view = {
            match view_model.current_view {
                View::Map => self.create_map_view(),
                View::Player => self.create_character_view(),
            }
        };

        let game = &view_model.game;
        let link = &view_model.link;

        let time_button = {
            let is_running = view_model.game_loop_task.is_some();
            let label = if is_running { "Pause" } else { "Resume" };
            let onclick = link.callback(move |_| {
                if is_running {
                    Msg::PauseGame
                } else {
                    Msg::ResumeGame
                }
            });

            html! {
                <button onclick=onclick>{label}</button>
            }
        };

        html! {
            <div>
                <div style={"display: flex"}>
                    <fieldset>
                        <legend>{"Views"}</legend>
                        <button onclick=link.callback(|_| Msg::SwitchView(View::Player))>{"Player"}</button>
                        <button onclick=link.callback(|_| Msg::SwitchView(View::Map))>{"Map"}</button>
                    </fieldset>
                    <fieldset>
                        <legend>{"Controls"}</legend>
                        {time_button}
                        <span>{"Time: "} {game.get_time()}</span>
                    </fieldset>
                </div>
                {view}
            </div>
        }
    }
}
