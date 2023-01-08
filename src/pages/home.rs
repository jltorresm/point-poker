use crate::app::Route;
use crate::components::galaxy::Galaxy;
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="d-flex flex-column align-items-center justify-content-center">
            <Galaxy/>
            <h1>{ "Point Poker" }</h1>
            <Link<Route> classes={classes!({"btn btn-outline-info btn-lg mt-1"})} to={Route::CreateSession}>{"Start Session"}</Link<Route>>
        </div>
    }
}
