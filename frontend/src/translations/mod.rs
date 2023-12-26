mod cell;
mod editor;
mod locales;
mod row;
use editor::Editor;
use leptos::*;

#[component]
pub fn Translations() -> impl IntoView {
    let is_project_open = true;

    if !is_project_open {
        return view! {
            <>
                <div class="flex flex-col items-center justify-center h-full w-full select-none">
                    <p class="text-2xl text-neutral">No project open</p>
                    <p class="text-neutral">Open a project to start translating</p>
                </div>
            </>
        };
    }

    view! {<>
        <Editor />
    </>}
}
