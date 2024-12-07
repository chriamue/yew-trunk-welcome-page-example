use routing::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Welcome page initialized");
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#welcome")
        .unwrap()
        .unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
