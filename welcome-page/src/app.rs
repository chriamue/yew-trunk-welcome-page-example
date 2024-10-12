use crate::{switch, Navigation, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Navigation />
            <Switch<Route> render={switch} />
        </>
    }
}
