use crate::{app::Route, game::Poker, hooks::use_app_state, state};
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[function_component(Create)]
pub fn create() -> Html {
    let app_state = use_app_state();

    // Create the new game
    let game = Poker::default();
    let game_id = game.id;
    let app_state = app_state.with_game(game);

    // Store the game in the state
    LocalStorage::set(state::KEY, app_state).expect("failed to persist state");

    html! {
        <Redirect<Route> to={Route::Join{ id: game_id }}/>
    }
}
