use std::{cell::RefCell, rc::Weak};

use yew::{prelude::*, Properties};

use super::{Character, CharacterId};
use crate::{app::ViewModel, CompleteCoor};

pub struct CharacterView {
    view_model: Weak<RefCell<ViewModel>>,
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct CharacterViewProps {
    pub view_model: Weak<RefCell<ViewModel>>,
}

impl PartialEq for CharacterViewProps {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Component for CharacterView {
    type Message = Msg;
    type Properties = CharacterViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            view_model: ctx.props().view_model,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_model = self
            .view_model
            .upgrade()
            .expect("cannot upgrade 'view_model' to Rc");
        let view_model = view_model.borrow();
        let game = &view_model.get_game();
        let player_character_id = game.player_character_id;
        let player_character = &game.characters[player_character_id];
        let characters = &game.characters;

        let parents = player_character.get_parents();
        let parents_rows = parents.iter().map(|id| {
            let index: usize = id.clone().into();
            let character = &characters[index];
            render_character(*id, character, 0)
        });

        let children_rows = player_character.get_children().map(|id| {
            let index: usize = id.clone().into();
            let character = &characters[index];
            render_character(*id, character, 0)
        });

        let known_people_rows = player_character.get_known_people().map(|(id, opinion)| {
            let index: usize = id.clone().into();
            let character = &characters[index];
            render_character(*id, character, 0)
        });

        let player_name = player_character.copy_name();
        let player_coor = player_character.get_coor();
        let player_character_id: usize = player_character_id.into();

        let player_info = format!("Player {} ({}) ", player_name, player_character_id);

        html! {
            <>
                {player_info} {render_coor(player_coor)}

                <div>
                    {"Parents"}
                    <table class="character">
                        <thead>
                            <tr>
                                <td>{"Id"}</td>
                                <td>{"Name"}</td>
                                <td>{"Location"}</td>
                                <td>{"Opinion"}</td>
                            </tr>
                        </thead>
                        <tbody>
                            {for parents_rows}
                        </tbody>
                    </table>
                </div>

                <div>
                    {"Children"}
                    <table class="character">
                        <thead>
                            <tr>
                                <td>{"Id"}</td>
                                <td>{"Name"}</td>
                                <td>{"Location"}</td>
                                <td>{"Opinion"}</td>
                            </tr>
                        </thead>
                        <tbody>
                            {for children_rows}
                        </tbody>
                    </table>
                </div>

                <div>
                    {"Other known people"}
                    <table class="character">
                        <thead>
                            <tr>
                                <td>{"Id"}</td>
                                <td>{"Name"}</td>
                                <td>{"Location"}</td>
                                <td>{"Opinion"}</td>
                            </tr>
                        </thead>
                        <tbody>
                            {for known_people_rows}
                        </tbody>
                    </table>
                </div>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {}
}

fn render_character(character_id: CharacterId, character: &Character, opinion: i8) -> Html {
    let name = character.copy_name();
    let coor = character.get_coor();
    let character_id: usize = character_id.into();
    html! {
        <tr>
            <td>{character_id}</td>
            <td>{name}</td>
            <td>{render_coor(coor)}</td>
            <td>{opinion}</td>
        </tr>
    }
}

fn render_coor(coor: CompleteCoor) -> Html {
    match coor {
        CompleteCoor::InSpace(universe_id, point) => {
            let universe_id: usize = universe_id.into();
            html! {
                <span>{"In Space "}{universe_id}{":"}{point}</span>
            }
        }
        CompleteCoor::OnPlanetRoad(universe_id, planet_id, point) => {
            let universe_id: usize = universe_id.into();
            let planet_id: usize = planet_id.into();
            html! {
                <span>{format!("On Planet (U:{},P:{},R:{})", universe_id, planet_id, point)}</span>
            }
        }
        CompleteCoor::OnPlanetRegion(universe_id, planet_id, region_id) => {
            let universe_id: usize = universe_id.into();
            let planet_id: usize = planet_id.into();
            let region_id: usize = region_id.into();
            html! {
                <span>{format!("On Planet (U:{},P:{},R:{})", universe_id, planet_id, region_id)}</span>
            }
        }
    }
}
