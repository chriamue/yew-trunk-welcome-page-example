pub struct MainApp;
use crate::{About, Home, NotFound, Route};
use welcome_page::Switch;
use yew::prelude::*;

impl Switch for MainApp {
    fn switch(&self, route: &Route) -> Html {
        match route {
            Route::Home => html! { <Home /> },
            Route::About => html! { <About /> },
            Route::Root => html! { <Home /> },
            _ => html! { <NotFound /> },
        }
    }
}
