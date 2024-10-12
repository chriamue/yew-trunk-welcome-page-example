use crate::{AppState, LoadMainApp, Navigation, Route, Switch as _, Welcome};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Welcome);
    let welcome_app = WelcomeApp;

    let switch = {
        let app_state = app_state.clone();
        move |route: Route| match *app_state {
            AppState::Welcome => welcome_app.switch(&route),
            AppState::Main => html! { <LoadMainApp /> },
        }
    };

    let onnavigation = {
        let app_state = app_state.clone();
        Callback::from(move |route: Route| {
            if matches!(route, Route::Home | Route::About) {
                app_state.set(AppState::Main);
            }
        })
    };

    html! {
        <BrowserRouter>
            <Navigation />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

pub struct WelcomeApp;

impl crate::Switch for WelcomeApp {
    fn switch(&self, route: &Route) -> Html {
        match route {
            Route::Welcome => html! { <Welcome /> },
            _ => html! { <LoadMainApp /> },
        }
    }
}
