use crate::state;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[hook]
pub fn use_app_state() -> UseStateHandle<state::State> {
    use_state_eq(|| {
        LocalStorage::get::<state::State>(state::KEY).unwrap_or(state::State::default())
    })
}

#[hook]
pub fn use_name() -> UseStateHandle<Option<String>> {
    let app_state = use_app_state();
    use_state_eq(|| app_state.name.clone())
}
