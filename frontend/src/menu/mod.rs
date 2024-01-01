mod project;
mod projects;
mod state;
use std::time::Duration;

use crate::{
    components::dropdown::{DropDown, DropDownItem},
    menu::projects::RecentProjects,
    settings::Settings,
};
use leptos::*;
#[component]
pub fn MenuBtn() -> impl IntoView {
    let menu_state = create_rw_signal(false);

    let open = move |_| menu_state.set(true);
    view! {
        <button class="w-10 pl-6" on:click=open >
            <i class="text-primary text-lg feather-menu"></i>
        </button>
        <Portal>
            <Menu menu_state/>
        </Portal>
    }
}

#[component]
fn Menu(menu_state: RwSignal<bool>) -> impl IntoView {
    let new_project_state = create_rw_signal(false);

    let close_menu = move |_| {
        menu_state.set(false);
        new_project_state.set(false);
    };

    view! {
        <AnimatedShow
            when=menu_state
            hide_delay=Duration::from_millis(0)
        >
            <div class="overlay fade-in" on:click=close_menu></div>
        </AnimatedShow>
        <AnimatedShow
            when=menu_state
            hide_delay=Duration::from_millis(0)
        >
            <div class="menu h-full fixed top-0 left-0 w-144 bg-secondary z-20 flex slide-in-left">
                <div class="my-8 mx-6 flex flex-col w-full">
                    <div class="titlebar flex items-center justify-between">
                        <h1 class="font-bold text-3xl text-primary">Locale.ly</h1>
                        <button on:click=close_menu>
                            <i class="feather-x text-primary text-lg"></i>
                        </button>
                    </div>

                    <div class="flex flex-1 mt-4">
                        <div class="projects w-80 h-full flex flex-col">
                            <ProjectBtns new_project_state/>
                            <Show
                                when=move || new_project_state.get()
                                fallback=|| view!{<RecentProjects />}
                            >
                                <NewProjectModal new_project_state/>
                            </Show>
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
fn ProjectBtns(new_project_state: RwSignal<bool>) -> impl IntoView {
    let toggle_new_project = move |_| {
        new_project_state.update(|s| *s = !*s);
    };

    view! {
        <div class="flex w-full">
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1" on:click=toggle_new_project>
                <i class="feather-plus text-primary"></i>
            </button>
            <div class="w-2"></div>
            <button class="bg-transparent border-text-primary border rounded-md py-2 flex-1">
                <i class="feather-folder text-primary"></i>
            </button>
        </div>
    }
}

#[component]
fn NewProjectModal(new_project_state: RwSignal<bool>) -> impl IntoView {
    let close_new_project = move |_| {
        new_project_state.set(false);
    };

    view! {
        <AnimatedShow
            when=true
            hide_delay=Duration::from_millis(0)
        >
            <div class="w-full h-auto bg-secondary flex flex-col mt-5">
                <div class="flex justify-between items-center py-2">
                    <h1 class="font-bold text-lg text-primary">New Project</h1>
                </div>
                <div class="flex flex-col">
                    <div class="flex-1 flex flex-col mt-4">
                        <span class="text-sm text-primary">Name</span>
                        <div class="mt-2"></div>
                        <input type="text" class="text-primary bg-primary rounded-lg py-3 px-4" />
                    </div>
                    <div class="flex-1 flex flex-col mt-6">
                        <span class="text-sm text-primary">Location</span>
                        <div class="mt-2"></div>
                        <div class="flex">
                            <input type="text" class="text-primary bg-primary rounded-lg py-3 px-4 flex-1" />
                            <button class="px-3">
                                <i class="feather-folder text-primary"></i>
                            </button>
                        </div>
                    </div>
                    <div class="flex-1 flex flex-col mt-6">
                        <SrcLocale />
                    </div>
                </div>
                <div class="flex flex-col mt-6">
                    <button class="px-10 py-3 bg-blue-500 text-white rounded-lg">
                        Create
                    </button>
                    <button class="px-10 py-3 bg-primary text-red-500 rounded-lg mt-4" on:click=close_new_project>
                        Cancel
                    </button>
                </div>
            </div>
        </AnimatedShow>
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
