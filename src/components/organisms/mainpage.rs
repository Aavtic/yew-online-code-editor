use yew::prelude::*;
use stylist::{yew::styled_component, style};
use std::ops::Deref;
use crate::components::atoms::{
    main_title::MainTitle, 
    main_heading::MainHeading, 
    text_area::TextArea, 
    run_button:: RunButton, 
    language_selector::LanguageSelector,
};
use gloo::console::log;
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use reqwasm::http::Request;


#[derive(Default, Clone, Serialize, Deserialize)]
struct Data {
    pub code: String,
    pub language: String,
}

#[derive(Default)]
struct OutputState {
    output: String
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
    let state = use_state(|| Data{language:"rust".to_string(), ..Data::default()});
    let output_state = use_state(|| OutputState::default());

    let cloned_state = state.clone();
    let editor_content_callback = Callback::from(move |code: String| {
        cloned_state.set(
            Data {code, ..cloned_state.deref().clone()}
        )
    });

    let value = output_state.clone();
    let button_callback =  {
        let state_clone = state.deref().clone();
        Callback::from(move |_| {
            let output_state = output_state.clone();
            let state_clone = state_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let state_clone = state_clone.clone();
                let response = Request::post("http://127.0.0.1:8081/api/v1")
                    .header("content-type", "application/json")
                    .body(to_string_pretty(&state_clone).unwrap())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let new_output_state = OutputState{output: response};
                output_state.set(new_output_state)
            });
        })
    };
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
            <TextArea name="Code Editor" handle_onchange = {editor_content_callback} />
            <br/>
            <RunButton label="Submit" onclick={button_callback}/>
            <LanguageSelector languages={languages} default="rust" callback={select_callback}/>
            <div>
                <h1>{value.output.clone()}</h1>
            </div>
        </div>
    }
}
