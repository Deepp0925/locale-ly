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
    view! {
        <div class="translation-table">
            <table class="mb-32 px-4">
                <Locales />
                <tbody>
                    <For
                        each=move || ((0..512).collect::<Vec<_>>())
                        // a unique key for each item
                        key=|item| *item
                        // renders each item to a view
                        children=|_item| {
                            view!{
                                <Row />
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Locales() -> impl IntoView {
    let locales = [
        "English (en)",
        "Spanish (es)",
        "French (fr)",
        "German (de)",
        "Italian (it)",
    ];
    view! {
         <thead>
            <tr>
                <th class="bg-primary px-3 py-4 text-primary">Keys / Locales</th> // Empty Corner Cell
                <For
                    each=move||locales
                    key=|locale| locale.to_owned()
                    children=|locale| {
                        view! {
                            <td class="min-w-96 max-w-96 py-4 px-3 bg-primary text-primary text-md font-bold">
                                <strong>{locale}</strong>
                            </td>
                        }
                    }
                />
            </tr>
        </thead>
    }
}
