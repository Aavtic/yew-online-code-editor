use yew::prelude::*;
use stylist::{yew::styled_component, style};
use std::ops::Deref;
use crate::components::atoms::{
    main_title::MainTitle, 
    main_heading::MainHeading, 
    text_area::TextArea, 
    run_button:: RunButton, 
    language_selector::LanguageSelector
};
use gloo::console::log;
use serde::Serialize;


#[derive(Default, Clone, Serialize)]
struct Data {
    pub code: String,
    pub button_clicked: u32,
    pub language: String,
}

#[styled_component(MainPage)]
pub fn main_page() -> Html {
    let languages = vec!["python".to_string(), "rust".to_string()];
    let stylesheet = style!(
        r#"
            h1 {
                color: yellow;
            }

            textarea {
                color: red;
                width: 400px;
                height: 200px;
            }
            
            button {
                
            }
        "#
        ).unwrap();
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |code: String| {
        cloned_state.set(
            Data {code, ..cloned_state.deref().clone()}
        )
    });

    let cloned_state = state.clone();
    let button_callback = Callback::from(move |_| {
        let current_state = cloned_state.deref();
        log!(serde_json::to_string_pretty(current_state).unwrap());
    });

    let cloned_state = state.clone();
    let select_callback = Callback::from( move |language: String| {
        log!("selected languages is ", &language);
        cloned_state.set(
            Data {language, ..cloned_state.deref().clone()}
        );
    });


    html! {
        <div class={stylesheet}>
            <MainTitle/>
            <MainHeading/>
            <br/>
            <TextArea name="Code Editor" handle_onchange = {username_changed} />
            <br/>
            <RunButton label="Submit" onclick={button_callback}/>
            <LanguageSelector languages={languages} default="rust" callback={select_callback}/>
        </div>
    }
}
