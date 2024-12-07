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

#[wasm_bindgen(js_name = render_app)]
pub fn render_app(root_selector: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log::info!("Rendering main app to selector: {}", root_selector);

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document found"))?;

    let root = document
        .query_selector(&root_selector)
        .map_err(|_| JsValue::from_str("Failed to query selector"))?
        .ok_or_else(|| JsValue::from_str("Root element not found"))?;

    yew::Renderer::<Main>::with_root(root).render();

    log::info!("Main app rendered successfully");
    Ok(())
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}
