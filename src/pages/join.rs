use crate::app::Route;
use crate::components::{galaxy::Galaxy, name_input::NameInput};
use crate::hooks::{use_app_state, use_name};
use crate::{game, state};
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub game_id: game::Id,
}

#[function_component(Join)]
pub fn join(props: &Props) -> Html {
    let Props { game_id } = props.clone();

    let navigator = use_navigator().expect("couldn't use the navigator");
    let app_state = use_app_state();
    let user_name_state = use_name();

    let on_name_change = use_callback(
        move |new_name: String, state_handle| {
            state_handle.set(Some(new_name));
        },
        user_name_state.clone(),
    );

    let onsubmit = move |e: SubmitEvent| {
        // Validate the entered name
        let name = match (*user_name_state).clone() {
            None => (true, String::new()),
            Some(n) => (n.is_empty(), n),
        };

        if name.0 {
            let _ = gloo::utils::window().alert_with_message("Invalid name!");
            e.prevent_default();
            return;
        }

        // Store the name for future reference
        let app_state = app_state.with_name(name.1);
        LocalStorage::set(state::KEY, app_state).expect("failed to persist state");
        e.prevent_default();

        navigator.push(&Route::Session { id: game_id });
    };

    html! {
        <form style="max-width:450px" class="d-flex flex-column m-auto" {onsubmit}>
            <Galaxy classes={classes!({"align-self-center"})} />
            <NameInput {on_name_change} />
            <button class="btn btn-info" type="submit">{"Join"}</button>
        </form>
    }
}
