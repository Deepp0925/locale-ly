mod cell;
mod editor;
mod row;
use editor::Editor;
use leptos::*;

use crate::project::ProjectState;

#[component]
pub fn Translations() -> impl IntoView {
    let state = expect_context::<RwSignal<ProjectState>>();
    let is_project_open = create_memo(move |_| state.get().is_project_open());

    if !is_project_open.get() {
        return view! {
            <div class="flex flex-col items-center justify-center h-full w-full select-none flex-1">
                <p class="text-2xl text-neutral">No project open</p>
                <p class="text-neutral">Open a project to begin translating</p>
            </div>
        };
    }

    view! {
        <div class="hide-x-scrollbar flex flex-1 overflow-auto w-full">
            <Editor />
        </div>
    }
}
