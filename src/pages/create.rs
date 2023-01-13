use crate::app::Route;
use crate::components::galaxy::Galaxy;
use crate::state;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Create)]
pub fn create() -> Html {
    let navigator = use_navigator().expect("couldn't use the navigator");

    let app_state_o = use_state_eq(|| {
        LocalStorage::get::<state::State>(state::KEY).unwrap_or(state::State::default())
    });
    let app_state = app_state_o.clone();

    let oninput = move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();

        app_state.set(app_state.with_name(value));
    };

    let onsubmit = move |e: SubmitEvent| {
        let is_bad = match app_state_o.name.clone() {
            None => true,
            Some(n) => n.is_empty(),
        };

        if is_bad {
            let _ = gloo::utils::window().alert_with_message("Invalid name!");
            e.prevent_default();
            return;
        }

        LocalStorage::set(state::KEY, (*app_state_o).clone()).expect("failed to persist state");
        e.prevent_default();

        // TODO: Create all the config for the game

        navigator.push(&Route::Session { id: 123456 });
    };

    html! {
        <form style="max-width:450px" class="d-flex flex-column m-auto" {onsubmit}>
            <Galaxy classes={classes!({"align-self-center"})} />

            <div class="mb-5">
                <label for="name" class="form-label display-6">{"Enter your name to join"}</label>
                <input type="text" class="form-control form-control-lg" placeholder="Hari Seldon"
                    {oninput} required={true}
                />
            </div>

            <button class="btn btn-info" type="submit">{"Join"}</button>
        </form>
    }
}
