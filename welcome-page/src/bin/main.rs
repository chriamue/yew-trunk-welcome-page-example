use welcome_page::App;

fn main() {
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#welcome")
        .unwrap()
        .unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
