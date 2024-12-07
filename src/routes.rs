use crate::{About, Home, ProjectDetails, Projects};
use routing::{NotFound, Welcome};
use yew::prelude::*;

pub use routing::Route;

pub fn switch(route: Route) -> Html {
    match route {
        Route::Welcome => html! { <Welcome /> },
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Projects => html! { <Projects /> },
        Route::Project { id } => html! { <ProjectDetails id={id} /> },
        Route::Root => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
