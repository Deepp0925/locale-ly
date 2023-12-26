use std::vec;

use leptos::*;

use super::row::Row;

/// Loading the editor
/// The editor encapsulates all the state regarding the current project open in
/// the application. It is responsible for loading the project, saving the project, and
/// managing the state of the project.
///
///
/// Locales (struct)
/// Hashmap
///     - key: String
///     - value: Translations (struct) contains src and all the translations

#[component]
pub fn Editor() -> impl IntoView {
    view! {<div class="flex-1 flex-col flex overflow-scroll">
        <Locales />
        <div class="flex-1 flex flex-col">
            <Row />
        </div>
    </div>}
}

#[component]
fn Locales() -> impl IntoView {
    let locales = ["English (en)", "Spanish (es)", "French (fr)"];
    view! {
         <div class="h-auto py-4 flex">
            <div class="w-96"></div> // Empty Corner Cell
            <For
                each=move||locales
                key=|locale| locale.to_owned()
                children=|locale| {
                    view! {
                        <div class="flex flex-col w-96 pb-3 px-3">
                            <p class="text-primary text-md mt-4 font-bold">{locale}</p>
                        </div>
                    }
                }
            />
        </div>
    }
}
