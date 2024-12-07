pub mod about;
pub mod app;
pub mod home;
mod main_app;
mod project_details;
mod projects;
mod routes;

pub use about::About;
pub use app::App;
pub use home::Home;
pub use main_app::MainApp;
pub use project_details::ProjectDetails;
pub use projects::Projects;
pub use routes::{switch, Route};
pub use routing::{config, Navigation, NotFound, Welcome};
