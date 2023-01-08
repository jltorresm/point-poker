use crate::components::galaxy::Galaxy;
use yew::prelude::*;

#[function_component(Create)]
pub fn create() -> Html {
    html! {
        <form style="max-width:450px" class="d-flex flex-column m-auto">
            <Galaxy classes={classes!({"align-self-center"})} />

            <div class="mb-5">
                <label for="name" class="form-label display-6">{"Enter your name to join"}</label>
                <input type="text" class="form-control form-control-lg" id="name" placeholder="Hari Seldon"/>
            </div>

            <button class="btn btn-info">{"Join"}</button>
        </form>
    }
}
