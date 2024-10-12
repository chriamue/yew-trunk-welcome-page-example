use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlScriptElement, Window};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn load_main_app_script() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    let script: HtmlScriptElement = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;

    script.set_type("module");
    script.set_text(
        r#"
        import('/main.js').then(module => {
            if (module.run_app) module.run_app();
        }).catch(e => console.error("Error loading main app:", e));
    "#,
    );

    let head = document.head().expect("document should have a head");
    head.append_child(&script)?;

    Ok(())
}

#[function_component(LoadMainApp)]
pub fn load_main_app() -> Html {
    let loaded = use_state(|| false);

    let loaded_clone = loaded.clone();
    use_effect_with((), move |_| {
        let loaded = loaded_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // Load main app bundle
            load_main_app_script().await;
            //loaded.set(true);
        });
        || ()
    });

    if *loaded {
        html! { <div id="main-app"></div> }
    } else {
        html! { <div>{"Loading..."}</div> }
    }
}
