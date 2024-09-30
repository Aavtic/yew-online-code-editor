use yew::prelude::*;

pub fn code_output(code_output: String) -> Html {
    html! {
        <p>
            <pre>
                <code>
                    {code_output}
                </code>
            </pre>
        </p>
    }
}


