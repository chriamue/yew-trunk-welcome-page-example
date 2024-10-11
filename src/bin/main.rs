use yew::prelude::*;
use yew_router::prelude::*;
use yew_trunk_welcome_page_example::App;

#[function_component(Main)]
fn main_app() -> Html {
    html! {
        <BrowserRouter>
            <App />
        </BrowserRouter>
    }
}

fn main() {
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#app")
        .unwrap()
        .unwrap();
    yew::Renderer::<Main>::with_root(root).render();
}
