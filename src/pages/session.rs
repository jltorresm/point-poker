use crate::game;
use crate::state;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: game::Id,
}

#[function_component(Session)]
pub fn session(props: &Props) -> Html {
    let Props { id, .. } = props.clone();

    let app_state = use_state_eq(|| {
        LocalStorage::get::<state::State>(state::KEY).unwrap_or(state::State::default())
    });

    html! {
        <div class="d-flex flex-column align-items-center justify-content-center">
            <h1>{format!("Hello {}!", (*app_state).clone().name.unwrap())}</h1>
            <h2>{"Game: "}{id}</h2>
        </div>
    }
}
