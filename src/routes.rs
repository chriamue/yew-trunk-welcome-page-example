use crate::about::About;
use crate::config::full_path;
use crate::home::Home;
use welcome_page::Welcome;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/p/welcome")]
    Welcome,
    #[at("/p")]
    Home,
    #[at("/p/about")]
    About,
    #[not_found]
    #[at("/p/*")]
    NotFound,
}

impl Route {
    pub fn to_path(&self) -> String {
        full_path(match self {
            Route::Root => "/",
            Route::Welcome => "/p/welcome",
            Route::Home => "/p",
            Route::About => "/p/about",
            Route::NotFound => "/p/404",
        })
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Root | Route::Welcome => html! { <Welcome /> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
