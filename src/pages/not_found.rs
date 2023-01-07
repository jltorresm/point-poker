use crate::app::Route;
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <article class="mt-5 text-center">
            <h1 class="mt-5 display-3 fw-bold">{"Page Not Found"}</h1>
            <p class="lead mb-4">{"let's go back "}
                <Link<Route> classes={classes!({"text-success-emphasis"})} to={Route::Home}>{"home"}</Link<Route>>
            </p>
            <br/>
            <img src="https://source.unsplash.com/random/800x450?lost,alone,sad"
                class="img-fluid border rounded-3 shadow-lg mb-4" alt="You are lost"
                loading="lazy" width="800" height="450"/>
        </article>
    }
}
