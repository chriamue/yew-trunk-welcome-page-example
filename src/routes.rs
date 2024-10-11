use crate::about::About;
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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Root => html! { <Welcome /> },
        Route::Welcome => html! { <Welcome /> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
