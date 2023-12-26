mod project;
mod projects;
mod state;
use std::time::Duration;

use crate::settings::Settings;
use chrono::Utc;
use leptos::*;
use project::RecentProject;
use projects::RecentProjects;
pub use state::*;
#[component]
pub fn Menu() -> impl IntoView {
    let recent_projects = vec![RecentProject {
        path: "/home/leptos/projects/leptos".to_string(),
        name: "Leptos".to_string(),
        last_opened: Utc::now(),
    }];

    let state = use_menu_read_signal();
    let is_open =
        MaybeSignal::derive(move || state.get().unwrap_or(MenuState::Closed) == MenuState::Open);

    let close_menu = |_| close_menu();

    // TODO add animation somehow
    view! {
        <AnimatedShow
            when=is_open
            hide_delay=Duration::from_millis(0)
        >
            <div class="overlay h-full w-full z-10 fixed" on:click=close_menu></div>
        </AnimatedShow>
        <AnimatedShow
            when=is_open
            hide_delay=Duration::from_millis(0)
        >
            <div class="menu h-full fixed left-0 w-144 bg-secondary z-20 flex">
                <div class="my-8 mx-6 flex flex-col w-full">
                    <div class="titlebar flex items-center justify-between">
                        <h1 class="font-bold text-3xl text-primary">Locale.ly</h1>
                        <button on:click=close_menu>
                            <i class="feather-x text-primary text-lg"></i>
                        </button>
                    </div>

                    <div class="flex flex-1 mt-4">
                        <div class="projects w-80 h-full flex flex-col">
                            <ProjectBtns/>
                            <RecentProjects projects=Some(recent_projects.clone())/>
                        </div>
                        <div class="divider my-16 mx-6 mr-8 bg-divider"></div>
                        <div class="settings flex-1 flex flex-col">
                            <Settings/>
                        </div>
                    </div>

                </div>
            </div>
        </AnimatedShow>

    }
}

#[component]
fn ProjectBtns() -> impl IntoView {
    view! {
        <div class="flex w-full">
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1">
                <i class="feather-plus text-primary"></i>
            </button>
            <div class="w-2"></div>
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1">
                <i class="feather-folder text-primary"></i>
            </button>
        </div>
    }
}
