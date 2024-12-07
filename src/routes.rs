use crate::{About, Home};
use routing::{NotFound, Welcome};
use yew::prelude::*;

pub use routing::Route;

pub fn switch(switch: Route) -> Html {
    match switch {
        Route::Welcome => html! { <Welcome /> },
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Root => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
