use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{"About"}</h1>
            <p>{"This is the about page."}</p>
            <h2>{"Features"}</h2>
            <ul>
                <li>{"Rust"}</li>
                <li>{"Yew"}</li>
                <li>{"WebAssembly"}</li>
            </ul>
        </div>
    }
}
