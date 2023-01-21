use crate::app::Route;
use crate::game::{self, Poker, VoteType};
use crate::hooks::use_app_state;
use crate::state;
use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: game::Id,
}

#[function_component(Session)]
pub fn session(props: &Props) -> Html {
    let Props { id, .. } = props.clone();

    let app_state = use_app_state();
    let game_state = use_state_eq(|| app_state.game.clone().unwrap_or(Poker::default()));
    let vote_state = use_state_eq(|| game_state.vote_type);

    if match app_state.name.clone() {
        None => true,
        Some(g) => g.is_empty(),
    } {
        return html! {<Redirect<Route> to={Route::Join{ id }}/>};
    }

    let other_voting_options: Html = VoteType::iter()
        .map(|vote_type| {
            let onclick = {
                let vote_state = vote_state.clone();
                let app_state = app_state.clone();
                let game_state = game_state.clone();
                Callback::from(move |_| {
                    vote_state.set(vote_type);
                    let game = (*game_state).with_vote_type(vote_type);
                    LocalStorage::set(state::KEY, app_state.with_game(game))
                        .expect("failed to persist state");
                })
            };

            let label = vote_type.to_string();
            html! {
                <div key={label.clone()} lass="me-2">
                    <input type="radio" class="btn-check"
                        name="vote_type" id={label.clone()}
                        autocomplete="off" checked={*vote_state == vote_type}
                    />
                    <label class="btn btn-outline-info" for={label} {onclick}>{vote_type}</label>
                </div>
            }
        })
        .collect();

    html! {
        <div class="d-flex flex-column align-items-center justify-content-center">
            <h2>{format!("Hi {}!", (*app_state).clone().name.unwrap())}</h2>
            <h1 class="display-3">{"Time to Vote"}</h1>
            <h2>{"Game: "}{id}</h2>

            <div class="btn-group my-4" role="group" aria-label="Default button group">
                {
                    (*vote_state).options().iter().map(|vote_type| {
                        html!{
                            <button type="button" class="btn btn-lg btn-outline-info">{vote_type}</button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <div class="d-flex flex-wrap justify-content-evenly w-50 border border-light p-4">
                {other_voting_options}
            </div>
        </div>
    }
}
