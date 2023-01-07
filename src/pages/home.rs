use crate::app::Route;
use rand::Rng;
use yew::prelude::*;
use yew_router::components::Link;

fn random_galaxy() -> String {
    let num = rand::thread_rng().gen_range(0..10);
    format!("/images/galaxy/{num}.jpeg")
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="d-flex flex-column align-items-center justify-content-center full-height">
            <img src={random_galaxy()}
                class="img-fluid border rounded-bottom-circle shadow-lg mb-5" alt=""
                loading="lazy" width="256" height="256"/>
            <h1>{ "Point Poker" }</h1>
            <Link<Route> classes={classes!({"btn btn-outline-info btn-lg mt-1"})} to={Route::Home}>{"Start Session"}</Link<Route>>
        </div>
    }
}
