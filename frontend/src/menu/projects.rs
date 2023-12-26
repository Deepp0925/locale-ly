use leptos::*;

use super::project::{Project, RecentProject};

pub type RecentProjects = Option<Vec<RecentProject>>;

#[component]
pub fn RecentProjects(projects: RecentProjects) -> impl IntoView {
    view! {
        <h3 class="text-primary text-md mt-4 text-neutral">Recent Projects</h3>
        <div class="mt-1"></div>
        <div class="flex flex-col w-full flex-1">
            <Projects projects=projects />
        </div>
    }
}

#[component]
fn Projects(projects: RecentProjects) -> impl IntoView {
    if projects.is_none() {
        return view! {
            <>
                <p class="text-neutral text-sm">No recent projects</p>
            </>
        };
    }

    let projects = projects.unwrap();

    view! {
        <>
            <For
                each=move ||projects.clone()
                key = |project|project.path.clone()
                children=|project| {
                    view! {
                        <Project project=project />
                    }
                }
            />

        </>
    }
}
