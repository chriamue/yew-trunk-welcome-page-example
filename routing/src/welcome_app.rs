use crate::routes::Route;
use crate::MainApp;
use crate::Welcome;
use yew::prelude::*;

pub struct WelcomeApp;

impl crate::Switch for WelcomeApp {
    fn switch(&self, route: &Route) -> Html {
        match route {
            Route::Root | Route::Welcome => html! { <Welcome /> },
            _ => html! { <MainApp /> },
        }
    }
}
