use crate::Route;
use serde_json;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::{Document, HtmlScriptElement, Window};
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    type MainApp;

    #[wasm_bindgen(static_method_of = MainApp, catch)]
    fn switch(route: String) -> Result<String, JsValue>;
}

async fn load_main_app_script() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    let script: HtmlScriptElement = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;

    script.set_type("module");
    let script_content = r#"
        import * as main_app from '/main.js';
        window.MainApp = main_app.MainApp;
    "#;
    script.set_text(script_content)?;

    let head = document.head().expect("document should have a head");
    head.append_child(&script)?;

    // Wait for the script to load
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        script.set_onload(Some(&resolve));
    });
    wasm_bindgen_futures::JsFuture::from(promise).await?;

    // Verify that MainApp is defined
    js_sys::eval(
        "if (typeof window.MainApp === 'undefined') throw new Error('MainApp not defined');",
    )?;

    Ok(())
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
                match load_main_app_script().await {
                    Ok(_) => loaded.set(true),
                    Err(e) => {
                        log(&format!("Failed to load main app: {:?}", e));
                        error.set(Some(format!("Failed to load main app: {:?}", e)));
                    }
                }
            });
            || ()
        });
    }

    let switch = Callback::from(move |route: Route| {
        let route_str = serde_json::to_string(&route).unwrap();
        match MainApp::switch(route_str) {
            Ok(html_str) => Html::from_html_unchecked(html_str.into()),
            Err(e) => {
                log(&format!("Error in MainApp::switch: {:?}", e));
                html! { <div>{"Error loading content"}</div> }
            }
        }
    });

    if *loaded {
        html! {
            <Switch<Route> render={switch} />
        }
    } else if let Some(err) = error.as_ref() {
        html! { <div>{err}</div> }
    } else {
        html! { <div>{"Loading..."}</div> }
    }
}
