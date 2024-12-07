use crate::{MainApp, Navigation, Route, Switch as _, WelcomeApp};
use yew::prelude::*;
use yew_router::prelude::*;

pub enum AppState {
    Welcome,
    Main,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Welcome);

    let welcome_app = WelcomeApp;

    let switch = {
        let app_state = app_state.clone();
        move |route: Route| match *app_state {
            AppState::Welcome => match route {
                Route::Root | Route::Welcome => html! {
                    <>
                    <Navigation />
                    {welcome_app.switch(&route)}
                    </>
                },
                _ => {
                    app_state.set(AppState::Main);
                    log::info!("Switching to main app {:?}", route);
                    html! { <MainApp /> }
                }
            },
            AppState::Main => html! { <MainApp /> },
        }
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
