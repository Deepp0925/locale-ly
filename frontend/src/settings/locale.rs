use leptos::*;

use crate::components::dropdown::{DropDown, DropDownItem};

#[component]
pub fn Locale() -> impl IntoView {
    let languages = vec![DropDownItem {
        text: "English (en)".to_string(),
        value: "en".to_string(),
        selected: true,
        disabled: false,
    }];
    view! {
        <h3 class="text-primary text-md mt-4">Language</h3>
        <div class="mt-2"></div>
        <DropDown
            items=languages
        />
    }
}
