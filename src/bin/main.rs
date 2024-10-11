use yew_trunk_welcome_page_example::App;

fn main() {
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#app")
        .unwrap()
        .unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
