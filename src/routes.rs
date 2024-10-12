use crate::{About, Home};
use welcome_page::{NotFound, Welcome};
use yew::prelude::*;

pub use welcome_page::Route;

pub fn switch(switch: Route) -> Html {
    match switch {
        Route::Welcome => html! { <Welcome /> },
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Root => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
