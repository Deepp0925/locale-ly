use crate::components::tab_selector::{Tab, TabSelector};
use leptos::*;

#[component]
pub fn Theme() -> impl IntoView {
    let tabs = [Tab::from("Light"), Tab::from("Dark"), Tab::from("System")];

    let on_change = |_index: usize| {};

    view! {
        <>
            <h3 class="text-primary mt-8">Theme</h3>
            <div class="mt-2"></div>
            <TabSelector tabs=tabs name="theme" active=2 on_change=on_change/>
        </>

    }
}
