use crate::config::BASE_PATH;
use crate::App;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Main)]
fn main_app() -> Html {
    html! {
        <BrowserRouter basename={BASE_PATH}>
            <App />
        </BrowserRouter>
    }
}

#[wasm_bindgen(js_name = renderApp)]
pub fn render_app() {
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#app")
        .unwrap()
        .unwrap();
    yew::Renderer::<Main>::with_root(root).render();
}

#[wasm_bindgen(start)]
pub fn run() {
    // Make render_app available in the global scope
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        fn set_render_app(f: JsValue);
    }

    let closure = Closure::wrap(Box::new(render_app) as Box<dyn Fn()>);
    set_render_app(closure.as_ref().clone());
    closure.forget(); // Leak the closure to keep it alive
}
