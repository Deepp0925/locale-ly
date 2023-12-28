use leptos::*;

// #[derive(Debug, Clone, Copy)]
// pub enum TranslationType {
//     /// The translation is automatically generated.
//     Auto,
//     /// The translation is manually written by the user so don't make any changes.
//     Manual,
// }

// pub struct CellInfo {
//     pub text: String,
//     pub translation: TranslationType,
// }
#[component]
pub fn HeaderCell(text: String) -> impl IntoView {
    view! {
        <th class="w-96 bg-primary align-top font-extrabold">
            <div class="w-full bg-transparent h-auto px-3 py-4 outline-none font-bold" contenteditable="true">
                {text}
            </div>
        </th>
    }
}

#[component]
pub fn Cell(text: String) -> impl IntoView {
    view! {
        <td class="align-top">
            <div class="w-full bg-transparent h-auto px-3 py-4 outline-none border-b border-transparent focus:border-blue-500 duration-200" contenteditable="true">
                {text}
            </div>
        </td>
    }
}
