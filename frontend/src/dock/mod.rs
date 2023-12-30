mod auto_translate;
mod locales;

use crate::export::ExportBtn;
use crate::locales::EditLocalesBtn;
use auto_translate::AutoTranslate;
use leptos::*;

use crate::menu::MenuBtn;

#[component]
pub fn Dock() -> impl IntoView {
    view! {
        <div class="flex h-16 w-full items-center bg-secondary">
            <MenuBtn />
            <div class="flex ml-4 flex-1">
                Project Name
            </div>
            <EditLocalesBtn />
            <AutoTranslate />
            <ExportBtn />
        </div>
    }
}
