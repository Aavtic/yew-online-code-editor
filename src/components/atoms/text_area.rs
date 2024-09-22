use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use stylist::yew::styled_component;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}



#[styled_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();

    let callback = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });

    html! {
        <form>
            <textarea name={props.name.clone()} onchange={callback} placeholder={props.name.clone()}>
            </textarea>
        </form>
    }
}

