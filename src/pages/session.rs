use crate::app::Route;
use crate::game;
use crate::hooks::use_app_state;
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

    if match app_state.name.clone() {
        None => true,
        Some(g) => g.is_empty(),
    } {
        return html! {<Redirect<Route> to={Route::Join{ id }}/>};
    }

    html! {
        <div class="d-flex flex-column align-items-center justify-content-center">
            <h1>{format!("Hello {}!", (*app_state).clone().name.unwrap())}</h1>
            <h2>{"Game: "}{id}</h2>
        </div>
    }
}
