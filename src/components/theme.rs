use crate::state::{self, Theme};
use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::prelude::*;

#[function_component(ThemePicker)]
pub fn theme_picker() -> Html {
    let app_state = use_state_eq(|| {
        LocalStorage::get::<state::State>(state::KEY).unwrap_or(state::State::default())
    });

    let select_theme = |new_theme: Theme| {
        let app_state = app_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            app_state.set(app_state.with_theme(new_theme));
        })
    };

    use_effect_with_deps(
        move |state| {
            LocalStorage::set(state::KEY, state.clone()).expect("failed to persist state");
            gloo::utils::document()
                .get_elements_by_tag_name("html")
                .item(0)
                .expect("couldn't get HTML element")
                .set_attribute(
                    "data-bs-theme",
                    state.clone().theme.to_string().to_lowercase().as_str(),
                )
                .expect("couldn't set global theme");
            || ()
        },
        (*app_state).clone(),
    );

    html! {
        <div class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#" data-bs-toggle="dropdown">{app_state.theme.icon()}</a>
            <ul class="dropdown-menu dropdown-menu-end">
            {
                Theme::iter().map(|theme_name| {
                    let label = format!("{} {theme_name}", theme_name.icon());
                    html!{
                        <li key={theme_name.to_string()}>
                            <a class="dropdown-item" href="#" onclick={select_theme(theme_name)}>{label}</a>
                        </li>
                    }
                }).collect::<Html>()
            }
            </ul>
        </div>
    }
}
