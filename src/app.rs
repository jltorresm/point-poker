use crate::components::nav::Nav;
use crate::game;
use crate::pages;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct Props {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container-fluid d-flex flex-column full-height">
            <HashRouter>
                <Nav />
                <main class="row flex-grow-1">
                    <Switch<Route> render={switch} />
                </main>
            </HashRouter>
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"
                integrity="sha384-w76AqPfDkMBDXo30jS1Sgez6pr3x5MlQ1ZAGC+nuZB+EYdgRZgiwxhTBTkF7CXvN"
                crossorigin="anonymous"></script>
        </div>
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/session")]
    CreateSession,
    #[at("/join/:id")]
    Join { id: game::Id },
    #[at("/s/:id")]
    Session { id: game::Id },
}

#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <pages::Home /> }
        }
        Route::NotFound => {
            html! { <pages::NotFound /> }
        }
        Route::CreateSession => {
            html! { <pages::Create /> }
        }
        Route::Join { id: game_id } => {
            html! { <pages::Join {game_id} /> }
        }
        Route::Session { id } => {
            html! { <pages::Session {id} /> }
        }
    }
}
