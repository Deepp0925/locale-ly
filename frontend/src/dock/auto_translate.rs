use leptos::*;

#[component]
pub fn AutoTranslate() -> impl IntoView {
    view! {
        <button class="flex h-16 w-20 bg-blue-500 items-center justify-center">
            <i class="feather-globe text-xl text-white"></i>
        </button>
    }
}
