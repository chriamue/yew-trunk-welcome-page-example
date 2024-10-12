use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <div>
            <h1>{"Welcome"}</h1>
            <p>{"This is the welcome page."}</p>
            <Link<Route> to={Route::Home}>{"Let's start"}</Link<Route>>
        </div>
    }
}
