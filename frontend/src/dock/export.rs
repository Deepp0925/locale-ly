use crate::export::open_export;
use leptos::*;
#[component]
pub fn Export() -> impl IntoView {
    let open_export = |_| open_export();

    view! {
        <button class="bg-primary-opposite text-primary-opposite h-full md:w-44 lg:w-56" on:click=open_export>
            Export
        </button>
    }
}
