use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_trunk_welcome_page_example::config::BASE_PATH;
use yew_trunk_welcome_page_example::App;

#[function_component(Main)]
fn main_app() -> Html {
    html! {
        <BrowserRouter basename={BASE_PATH}>
            <App />
        </BrowserRouter>
    }
}

#[wasm_bindgen]
pub fn render_app(root_selector: String) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let root = document
        .query_selector(&root_selector)
        .expect("Failed to find root element")
        .expect("Root element is None");

    yew::Renderer::<Main>::with_root(root).render();
}

pub fn main() {}
