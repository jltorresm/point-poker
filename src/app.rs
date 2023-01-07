use crate::components::nav::Nav;
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
        <div class="container-fluid">
            <BrowserRouter>
                <Nav />
                <main class="row">
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
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
    // #[at("/s/:id")]
    // Session { id: u32 },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <pages::Home /> }
        }
        Route::NotFound => {
            html! { <pages::NotFound /> }
        } // Route::Session { id } => {
          //     html! { <Post seed={id} /> }
          // }
    }
}
