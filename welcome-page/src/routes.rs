use crate::config::full_path;
use serde::Serialize;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Serialize, Debug)]
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
