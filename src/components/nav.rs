use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg row" role="navigation" aria-label="main navigation">
            <div class="container-fluid">
                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>{"Point Poker"}</Link<Route>>

                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#main-nav"
                    aria-controls="main-nav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class="collapse navbar-collapse" id="main-nav">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <Link<Route> classes={classes!("nav-link")} to={Route::Home}>{ "Home" }</Link<Route>>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
