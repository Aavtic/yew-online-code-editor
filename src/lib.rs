use yew::prelude::*;

mod components;
// pub mod axum_web_server;

use components::organisms::mainpage::MainPage;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <MainPage/>
    }
}


