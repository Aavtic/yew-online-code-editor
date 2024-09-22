use yew::prelude::*;

mod components;

use components::organisms::mainpage::MainPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <MainPage/>
    }
}

