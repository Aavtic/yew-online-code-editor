use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)] 
pub struct Props {
    pub languages: Vec<String>,
    pub default: String,
    pub callback: Callback<String>
}


#[function_component(LanguageSelector)] 
pub fn language_selector(props: &Props) -> Html {
    let state_clone = props.callback.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        state_clone.emit(value);
    });

    html! {
        <select name="languages" id="languages" onchange={onchange}>
        {props.languages.iter().map(|language| html! {<option value={language.clone()}> {language} </option>}).collect::<Html>()}
        </select>
    }
}
