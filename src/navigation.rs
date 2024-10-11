use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav>
            <ul>
                <li><Link<Route> to={Route::Welcome}>{"Welcome"}</Link<Route>></li>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
            </ul>
        </nav>
    }
}
