use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[function_component(ProjectCard)]
fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <div class="project-card">
            <h3>{&props.title}</h3>
            <p>{&props.description}</p>
            <Link<Route> to={Route::Project { id: props.id.clone() }}>
                {"View Details"}
            </Link<Route>>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        (
            "rust-wasm",
            "Rust WASM Project",
            "A WebAssembly project using Rust",
        ),
        (
            "yew-app",
            "Yew Application",
            "A frontend application using Yew",
        ),
        (
            "trunk-demo",
            "Trunk Demo",
            "Demonstration of Trunk capabilities",
        ),
    ];

    html! {
        <div class="projects-page">
            <h1>{"Projects"}</h1>
            <div class="projects-grid">
                {projects.into_iter().map(|(id, title, desc)| {
                    html! {
                        <ProjectCard
                            id={id.to_string()}
                            title={title.to_string()}
                            description={desc.to_string()}
                        />
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
