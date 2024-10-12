use crate::Route;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = renderApp)]
    fn render_app();
}

#[function_component(LoadMainApp)]
pub fn load_main_app() -> Html {
    let loaded = use_state(|| false);
    let error = use_state(|| None);

    {
        let loaded = loaded.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match std::panic::catch_unwind(|| render_app()) {
                    Ok(_) => {
                        // Hide the welcome div and show the app div
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let welcome_div = document.query_selector("#welcome").unwrap().unwrap();
                        let app_div = document.query_selector("#app").unwrap().unwrap();

                        welcome_div
                            .set_attribute("style", "display: none;")
                            .unwrap();
                        app_div.set_attribute("style", "display: block;").unwrap();

                        loaded.set(true);
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to render main app: {:?}", e);
                        log(&error_msg);
                        error.set(Some(error_msg));
                    }
                }
            });
            || ()
        });
    }

    if *loaded {
        // We won't return anything here since the main app is loaded in a separate container.
        html! {}
    } else if let Some(err) = error.as_ref() {
        html! { <div>{err}</div> }
    } else {
        html! { <div>{"Loading..."}</div> }
    }
}
