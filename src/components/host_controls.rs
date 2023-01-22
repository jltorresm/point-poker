use crate::game::VoteType;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_vote_type_change: Callback<VoteType>,
    pub selected: VoteType,
}

#[function_component(HostControls)]
pub fn host_controls(props: &Props) -> Html {
    let Props {
        on_vote_type_change,
        selected,
    } = props.clone();

    let voting_options: Html = VoteType::iter()
        .map(|vote_type| {
            let label = vote_type.to_string();
            let onclick = {
                let on_vote_type_change = on_vote_type_change.clone();
                move |_| on_vote_type_change.emit(vote_type)
            };

            html! {
                <div key={label.clone()} class="col text-center">
                    <input type="radio" class="btn-check"
                        name="vote_type" id={label.clone()}
                        autocomplete="off" checked={selected == vote_type}
                    />
                    <label class="btn btn-outline-info" for={label} {onclick}>{vote_type}</label>
                </div>
            }
        })
        .collect();

    html! {
        <div class="card border-danger-subtle w-50">
            <div class="card-body">
                <h5 class="card-title mb-4">{"Host Controls"}</h5>
                <h6 class="card-subtitle mb-2 text-muted">{"Change Vote Type"}</h6>
                <div class="row">{voting_options}</div>
            </div>
        </div>
    }
}
