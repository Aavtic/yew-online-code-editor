use yew::prelude::*;
use stylist::{yew::styled_component, style};
use crate::components::atoms::{
    main_title::MainTitle, 
    main_heading::MainHeading, 
};
use crate::components::molecules::code_editor_base::CodeEditor;



#[styled_component(MainPage)]
pub fn main_page() -> Html {

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

    html! {
        <div class={stylesheet}>
            <MainTitle/>
            <MainHeading/>
            <CodeEditor/>
        </div>
    }
}
