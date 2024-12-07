use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum AppJsState {
    NotLoaded,
    Loading,
    Loaded,
    Error(String),
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let app_js_state = use_state(|| AppJsState::NotLoaded);
    log::debug!("AppJsState: {:?}", *app_js_state);

    {
        let app_js_state = app_js_state.clone();

        use_effect_with((), move |_| {
            if *app_js_state == AppJsState::NotLoaded {
                app_js_state.set(AppJsState::Loading);

                log::debug!("Loading main app script...");

                wasm_bindgen_futures::spawn_local(async move {
                    let window = web_sys::window().expect("no global `window` exists");

                    log::info!("Loading main app...");
                    // Wait for render_app to be available
                    while js_sys::Reflect::get(&window, &JsValue::from_str("render_app")).is_err() {
                        gloo_timers::future::TimeoutFuture::new(150).await;
                    }

                    match js_sys::Reflect::get(&window, &JsValue::from_str("render_app")) {
                        Ok(render_app_fn) => {
                            log::info!("render_app function found");
                            let render_app: js_sys::Function = render_app_fn.dyn_into().unwrap();
                            let _ = render_app.call1(&JsValue::NULL, &JsValue::from_str("#app"));

                            app_js_state.set(AppJsState::Loaded);
                        }
                        Err(e) => {
                            app_js_state.set(AppJsState::Error(format!(
                                "Failed to get render_app function: {:?}",
                                e
                            )));
                        }
                    }
                });
            }
            || ()
        });
    }

    match (*app_js_state).clone() {
        AppJsState::Loaded => html! { <div id="app"></div> },
        AppJsState::Error(err) => html! { <div id="app">{"Error loading main app: "}{err}</div> },
        AppJsState::Loading => html! { <div id="app">{"Loading..."}</div> },
        AppJsState::NotLoaded => html! { <div id="app">{"Preparing to load main app..."}</div> },
    }
}
