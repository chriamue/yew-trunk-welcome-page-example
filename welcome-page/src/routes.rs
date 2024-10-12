use crate::config::full_path;
use crate::{NotFound, Welcome};
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
        Route::Home => html! {  <NotFound />  },
        Route::About => html! {  <NotFound />  },
        Route::Root | Route::Welcome => html! { <Welcome /> },
        Route::NotFound => html! { <NotFound />  },
    }
}
