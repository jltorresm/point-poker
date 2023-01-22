use crate::app::Route;
use crate::components::host_controls::HostControls;
use crate::game::{self, Poker, VoteType};
use crate::hooks::use_app_state;
use crate::state;
use gloo::storage::{LocalStorage, Storage};
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

    let on_vote_type_change = {
        let app_state = app_state.clone();
        let game_state = game_state.clone();
        let vote_state = vote_state.clone();
        Callback::from(move |new_vote_type: VoteType| {
            vote_state.set(new_vote_type);
            let game = (*game_state).with_vote_type(new_vote_type);
            LocalStorage::set(state::KEY, app_state.with_game(game))
                .expect("failed to persist state");
        })
    };

    html! {
        <div class="d-flex flex-column align-items-center justify-content-center">
            <h2>{format!("Hi {}!", (*app_state).clone().name.unwrap())}</h2>
            <h1 class="display-3">{"Time to Vote"}</h1>
            <h2>{"Game: "}{id}</h2>

            <div class="btn-group my-4" role="group" aria-label="Default button group">
                {
                    (*vote_state).options().iter().map(|vote| {
                        let onclick = {
                            let vote = *vote;
                            move |_| web_sys::console::log_1(&vote.to_string().into())
                        };
                        html!{
                            <button type="button" class="btn btn-lg btn-outline-info" {onclick}>
                                {vote}
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            <HostControls selected={*vote_state} {on_vote_type_change} />
        </div>
    }
}
