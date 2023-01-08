use rand::Rng;
use yew::prelude::*;

fn random_galaxy() -> String {
    let num = rand::thread_rng().gen_range(0..10);
    format!("/images/galaxy/{num}.jpeg")
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(256)]
    pub width: i32,
    #[prop_or(256)]
    pub height: i32,
}

#[function_component(Galaxy)]
pub fn galaxy(props: &Props) -> Html {
    let Props {
        mut classes,
        width,
        height,
    } = props.clone();

    classes.extend(vec![
        "img-fluid",
        "border",
        "rounded-bottom-circle",
        "shadow-lg",
        "mb-5",
    ]);

    html! {
        <img
            src={random_galaxy()}
            class={classes}
            alt=""
            loading="lazy"
            width={width.to_string()}
            height={height.to_string()}
        />
    }
}
