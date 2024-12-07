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
                    let mut attempts = 0;
                    const MAX_ATTEMPTS: i32 = 20;

                    while attempts < MAX_ATTEMPTS {
                        log::debug!("Checking for render_app...{}", attempts);
                        match js_sys::Reflect::get(&window, &JsValue::from_str("render_app")) {
                            Ok(render_app_value) => {
                                gloo_timers::future::TimeoutFuture::new(20).await;
                                if render_app_value.is_function() {
                                    log::info!("Found render_app function");
                                    let render_app =
                                        render_app_value.unchecked_into::<js_sys::Function>();

                                    match render_app
                                        .call1(&JsValue::NULL, &JsValue::from_str("#app"))
                                    {
                                        Ok(_) => {
                                            log::info!("Successfully called render_app");
                                            app_js_state.set(AppJsState::Loaded);
                                            return;
                                        }
                                        Err(e) => {
                                            let error_msg =
                                                format!("Failed to call render_app: {:?}", e);
                                            log::error!("{}", error_msg);
                                            app_js_state.set(AppJsState::Error(error_msg));
                                            return;
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                attempts += 1;
                                log::debug!(
                                    "Waiting for render_app... attempt {}/{}",
                                    attempts,
                                    MAX_ATTEMPTS
                                );
                                gloo_timers::future::TimeoutFuture::new(200).await;
                            }
                        }
                    }

                    app_js_state.set(AppJsState::Error("Timeout waiting for render_app".into()));
                });
            }
            || ()
        });
    }

    match (*app_js_state).clone() {
        AppJsState::Loaded => html! { <div id="app"></div> },
        AppJsState::Error(err) => html! {
            <div id="app" class="error">
                <h2>{"Error Loading Main Application"}</h2>
                <p>{err}</p>
                <p>{"Please try refreshing the page."}</p>
            </div>
        },
        AppJsState::Loading => html! {
            <div id="app" class="loading">
                <p>{"Loading main application..."}</p>
            </div>
        },
        AppJsState::NotLoaded => html! {
            <div id="app" class="not-loaded">
                <p>{"Preparing to load main application..."}</p>
            </div>
        },
    }
}
