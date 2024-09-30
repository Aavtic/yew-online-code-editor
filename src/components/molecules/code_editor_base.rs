use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use reqwasm::http::Request;
use stylist::yew::styled_component;
use std::ops::Deref;

use crate::components::atoms::{
    text_area::TextArea,
    run_button::RunButton,
    language_selector::LanguageSelector,
    code_output::code_output,
};


#[derive(Default, Clone, Serialize, Deserialize)]
struct Data {
    pub code: String,
    pub language: String,
}

#[derive(Default)]
struct OutputState {
    output: String
}


#[styled_component(CodeEditor)]
pub fn code_editor() -> Html {
    let languages = vec!["python".to_string(), "rust".to_string()];
    let state = use_state(|| Data{language:"rust".to_string(), ..Data::default()});
    let output_state = use_state(|| OutputState::default());

    let cloned_state = state.clone();
    let editor_content_callback = Callback::from(move |code: String| {
        cloned_state.set(
            Data {code, ..cloned_state.deref().clone()}
        )
    });

    let button_callback =  {
        let output_state = output_state.clone();
        let state_clone = state.deref().clone();
        Callback::from(move |_| {
            let output_state = output_state.clone();
            let state_clone = state_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
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
                output_state.set(new_output_state);
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
        <div>
            <br/>
            <TextArea name="Code Editor" handle_onchange = {editor_content_callback} />
            <br/>
            <RunButton label="Submit" onclick={button_callback}/>
            <LanguageSelector languages={languages} default="rust" callback={select_callback}/>
            {code_output(output_state.deref().output.clone())}
        </div>
    }
}
