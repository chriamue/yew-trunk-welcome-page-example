use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <div>
            <h1>{"Welcome"}</h1>
            <p>{"This is the welcome page."}</p>
            <a href="p">{"Let's start"}</a>
        </div>
    }
}
