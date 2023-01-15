use crate::hooks::use_name;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_name_change: Callback<String>,
    #[prop_or(false)]
    pub required: bool,
}

#[function_component(NameInput)]
pub fn name_input(props: &Props) -> Html {
    let Props {
        on_name_change,
        required,
    } = props.clone();

    let name_state = use_name();

    let oninput = move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        on_name_change.emit(value);
    };

    html! {
        <div class="mb-5">
            <label for="name" class="form-label display-6">{"Enter your name to join"}</label>
            <input type="text" class="form-control form-control-lg" placeholder="Hari Seldon"
                {required} value={(*name_state).clone()} {oninput}
            />
        </div>
    }
}
