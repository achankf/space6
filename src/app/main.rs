use gloo_timers::callback::Interval;
use yew::{function_component, html, prelude::*, ContextProvider};

use super::character_view::CharacterView;
use super::map_view::MapView;

use super::{Action, Model, View, ViewModelContext};

#[function_component(App)]
pub fn main_app() -> Html {
    let view_model = use_reducer(Model::default);
    let game_loop_task_id = use_state(|| -> Option<i32> { None });
    let game_loop_refresh_trigger = use_state(|| {});

    use_effect({
        let view_model = view_model.clone();

        move || {
            if let Some(handle) = *game_loop_task_id {
                if !view_model.should_game_loop_run {
                    let window = web_sys::window().unwrap();
                    window.clear_interval_with_handle(handle);
                    game_loop_task_id.set(None);
                }
            } else if view_model.should_game_loop_run {
                let tick_per_second = 12;
                let frequency = 1000 / tick_per_second;

                let view_model = view_model.clone();

                let interval = Interval::new(frequency, move || {
                    view_model.game.borrow_mut().progress();
                    game_loop_refresh_trigger.set(());
                });
                game_loop_task_id.set(Some(interval.forget()));
            }

            || {}
        }
    });

    let time_button = {
        let view_model = view_model.clone();
        let should_game_loop_run = view_model.should_game_loop_run;
        let label = if should_game_loop_run {
            "Pause"
        } else {
            "Resume"
        };
        let onclick = move |_| {
            if should_game_loop_run {
                view_model.dispatch(Action::PauseGame);
            } else {
                view_model.dispatch(Action::ResumeGame);
            };
        };

        html! {
            <button onclick={onclick}>{label}</button>
        }
    };

    let view = {
        match view_model.current_view {
            View::Map => html! {<MapView/>},
            View::Player => html! {<CharacterView/>},
        }
    };

    let game = view_model.game.borrow();

    html! {
        <ContextProvider<ViewModelContext> context={view_model.clone()}>
            <div>
                <div style={"display: flex"}>
                    <fieldset>
                        <legend>{"Views"}</legend>
                        <button onclick={
                            let view_model = view_model.clone();
                            move |_| view_model.dispatch(Action::SwitchView(View::Player))
                        }>{"Player"}</button>
                        <button onclick={
                            let view_model = view_model.clone();
                            move |_| view_model.dispatch(Action::SwitchView(View::Map))
                        }>{"Map"}</button>
                    </fieldset>
                    <fieldset>
                        <legend>{"Controls"}</legend>
                        {time_button}
                        <span>{"Time: "} {game.get_time()}</span>
                    </fieldset>
                </div>
                {view}
            </div>
        </ContextProvider<ViewModelContext>>
    }
}
/*
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let game = Game::create();
        let view_model = ViewModel::new(game);
        let view_model = Rc::new(RefCell::new(view_model));

        Self { view_model }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut view_model = self.view_model.borrow_mut();

        match msg {
            Msg::UpdatePlanetId(id) => view_model.try_select_planet(id),
            Msg::UpdateUniverseId(id) => view_model.try_select_universe(id),
            Msg::UpdateRegionId(id) => view_model.try_select_region(id),
            Msg::SwitchView(view) => view_model.switch_view(view),
            Msg::PauseGame => view_model.try_pause_game(),
            Msg::ResumeGame => view_model.try_resume_game(ctx),
            Msg::GameTick => view_model.progress_game_tick(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_model = use_state(|| {
            let game = Game::create();
            ViewModel::new(game)
        });

        let view = {
            match view_model.current_view {
                View::Map => self.create_character_view(), //self.create_map_view(ctx),
                View::Player => self.create_character_view(),
            }
        };

        let game = view_model.game.borrow();
        let link = ctx.link();

        let time_button = {
            let is_running = view_model.game_loop_task_id.is_some();
            let label = if is_running { "Pause" } else { "Resume" };
            let onclick = link.callback(move |_| {
                if is_running {
                    Msg::PauseGame
                } else {
                    Msg::ResumeGame
                }
            });

            html! {
                <button onclick={onclick}>{label}</button>
            }
        };

        html! {
            <ContextProvider<ViewModel> context={(*view_model).clone()}>
                <div>
                    <div style={"display: flex"}>
                        <fieldset>
                            <legend>{"Views"}</legend>
                            <button onclick={link.callback(|_| Msg::SwitchView(View::Player))}>{"Player"}</button>
                            <button onclick={link.callback(|_| Msg::SwitchView(View::Map))}>{"Map"}</button>
                        </fieldset>
                        <fieldset>
                            <legend>{"Controls"}</legend>
                            {time_button}
                            <span>{"Time: "} {game.get_time()}</span>
                        </fieldset>
                    </div>
                    {view}
                </div>
            </ContextProvider<ViewModel>>
        }
    }
}

 */
