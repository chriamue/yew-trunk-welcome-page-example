use yew_trunk_welcome_page_example::Welcome;

fn main() {
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#welcome")
        .unwrap()
        .unwrap();
    yew::Renderer::<Welcome>::with_root(root).render();
}
