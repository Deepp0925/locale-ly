mod project;
mod projects;
mod state;
use std::time::Duration;

use crate::{
    components::dropdown::{DropDown, DropDownItem},
    settings::Settings,
};
use chrono::Utc;
use leptos::*;
use project::RecentProject;
use projects::RecentProjects;
#[component]
pub fn MenuBtn() -> impl IntoView {
    let state = create_rw_signal(true);

    let open = move |_| state.set(true);
    view! {
        <button class="w-10 pl-6" on:click=open >
            <i class="text-primary text-lg feather-menu"></i>
        </button>
        <Portal>
            <Menu state />
        </Portal>
    }
}

#[component]
fn Menu(state: RwSignal<bool>) -> impl IntoView {
    let close = move |_| {
        state.set(false);
    };

    let recent_projects = vec![RecentProject {
        path: "/home/leptos/projects/leptos".to_string(),
        name: "Leptos".to_string(),
        last_opened: Utc::now(),
    }];

    view! {
        <AnimatedShow
            when=state
            hide_delay=Duration::from_millis(0)
        >
            <div class="overlay fade-in" on:click=close></div>
        </AnimatedShow>
        <AnimatedShow
            when=state
            hide_delay=Duration::from_millis(0)
        >
            <div class="menu h-full fixed top-0 left-0 w-144 bg-secondary z-20 flex slide-in-left">
                <div class="my-8 mx-6 flex flex-col w-full">
                    <div class="titlebar flex items-center justify-between">
                        <h1 class="font-bold text-3xl text-primary">Locale.ly</h1>
                        <button on:click=close>
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
    let state = create_rw_signal(true);

    let open = move |_| state.set(true);

    view! {
        <div class="flex w-full">
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1" on:click=open>
                <i class="feather-plus text-primary"></i>
            </button>
            <NewProjectModal state/>
            <div class="w-2"></div>
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1">
                <i class="feather-folder text-primary"></i>
            </button>
        </div>
    }
}

#[component]
fn NewProjectModal(state: RwSignal<bool>) -> impl IntoView {
    let close = move |_| {
        state.set(false);
    };

    view! {
        <Portal>
            <AnimatedShow
                when=state
                hide_delay=Duration::from_millis(0)
            >
                <div class="overlay fade-in z-40" on:click=close></div>
            </AnimatedShow>
            <AnimatedShow
                when=state
                hide_delay=Duration::from_millis(0)
            >
                <div class="w-full fixed top-0 left-0 h-auto bg-secondary z-50 flex flex-col slide-in-down py-8 px-6">
                    <div class="flex justify-between items-center py-2">
                        <h1 class="font-bold text-xl text-primary">New Project</h1>
                    </div>
                    <div class="h-5"></div>
                    <div class="flex">
                        <div class="flex-1 flex flex-col">
                            <span class="text-sm text-primary">Name</span>
                            <div class="mt-2"></div>
                            <input type="text" class="text-primary bg-primary rounded-lg py-3 px-4" />
                        </div>
                        <div class="md:w-5 lg:w-10"></div>
                        <div class="flex-1 flex flex-col">
                            <span class="text-sm text-primary">Location</span>
                            <div class="mt-2"></div>
                            <div class="flex">
                                <input type="text" class="text-primary bg-primary rounded-lg py-3 px-4 flex-1" />
                                <button class="px-3">
                                    <i class="feather-folder text-primary"></i>
                                </button>
                            </div>
                        </div>
                        <div class="md:w-5 lg:w-10"></div>
                        <div class="flex-1 flex flex-col">
                            <SrcLocale />
                        </div>
                    </div>
                    <div class="h-5"></div>
                    <div class="flex justify-end">
                        <button class="px-10 py-3 bg-primary text-red-500 rounded-lg">
                            Cancel
                        </button>
                        <div class="w-5"></div>
                        <button class="px-10 py-3 bg-blue-500 text-white rounded-lg">
                            Create
                        </button>
                    </div>
                </div>
            </AnimatedShow>
        </Portal>
    }
}

#[component]
fn SrcLocale() -> impl IntoView {
    let languages = vec![DropDownItem {
        text: "English (en)".to_string(),
        value: "en".to_string(),
        selected: true,
        disabled: false,
    }];
    view! {
        <h3 class="text-primary text-sm">Source Locale</h3>
        <div class="mt-2"></div>
        <DropDown
            items=languages
        />
    }
}
