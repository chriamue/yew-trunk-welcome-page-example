use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/welcome")]
    Welcome,
    #[at("/home")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/*")]
    NotFound,
}

impl Route {
    pub fn to_path(&self) -> String {
        let base_path = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.base_uri().ok())
            .flatten()
            .unwrap_or_default();

        let path = match self {
            Route::Root => "/",
            Route::Welcome => "/welcome",
            Route::Home => "/home",
            Route::About => "/about",
            Route::NotFound => "/404",
        };

        match base_path.eq("/") {
            true => path.to_string(),
            false => format!("{}{}", base_path, path),
        }
    }
}
