use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectDetailsProps {
    pub id: String,
}

#[function_component(ProjectDetails)]
pub fn project_details(props: &ProjectDetailsProps) -> Html {
    let project_data = use_state(|| None::<ProjectData>);

    {
        let id = props.id.clone();
        let project_data = project_data.clone();

        use_effect_with((), move |_| {
            let data = ProjectData {
                id: id.clone(),
                title: format!("Project {}", id),
                description: format!("Detailed description for project {}", id),
                created_at: "2024-01-01".to_string(),
                technologies: vec!["Rust".into(), "WebAssembly".into(), "Yew".into()],
            };

            project_data.set(Some(data));
        });
    }

    html! {
        <div class="project-details">
            <Link<Route> to={Route::Projects}>{"← Back to Projects"}</Link<Route>>

            if let Some(data) = (*project_data).as_ref() {
                <div class="project-content">
                    <h1>{&data.title}</h1>
                    <p class="description">{&data.description}</p>
                    <div class="metadata">
                        <p><strong>{"Created: "}</strong>{&data.created_at}</p>
                        <div class="technologies">
                            <strong>{"Technologies: "}</strong>
                            {data.technologies.iter().map(|tech| {
                                html! {
                                    <span class="tech-tag">{tech}</span>
                                }
                            }).collect::<Html>()}
                        </div>
                    </div>
                </div>
            } else {
                <div class="loading">
                    <p>{"Loading project details..."}</p>
                </div>
            }
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct ProjectData {
    id: String,
    title: String,
    description: String,
    created_at: String,
    technologies: Vec<String>,
}
