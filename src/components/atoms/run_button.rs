use yew::prelude::*;
use stylist::yew::styled_component;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
}


#[styled_component(RunButton)]
pub fn run_button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });
    html! {
        <button type="submit" onclick={button_onclick}> {props.label.clone()} </button>
    }
}

