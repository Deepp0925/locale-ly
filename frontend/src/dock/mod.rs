mod auto_translate;
mod export;
mod locales;

use auto_translate::AutoTranslate;
use export::Export;
use leptos::*;
use locales::Locales;

use crate::menu::open_menu;

#[component]
pub fn Dock() -> impl IntoView {
    let open_menu = |_| open_menu();
    view! {
        <div class="flex h-16 w-full items-center bg-secondary">
            <button class="w-10 pl-6" on:click=open_menu >
                <i class="text-primary text-lg feather-menu"></i>
            </button>
            <div class="flex ml-4 flex-1">
                Project Name
            </div>
            <Locales />
            <AutoTranslate />
            <Export />
        </div>
    }
}
