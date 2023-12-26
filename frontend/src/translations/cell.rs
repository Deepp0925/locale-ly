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
pub fn Cell(text: String) -> impl IntoView {
    view! {
        <div class="mb-4 px-3 w-96 py-2">{text}</div>
    }
}
