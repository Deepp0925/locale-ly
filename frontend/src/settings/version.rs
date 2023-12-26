use leptos::*;

#[component]
pub fn Version() -> impl IntoView {
    view! {
        <>
            <h3 class="text-primary mt-8">Version</h3>
            <div class="mt-2">
                <Update />
            </div>
            <div class="mt-1">
                <CurrentVersion />
                <Copyright />
            </div>
        </>

    }
}

#[component]
fn CurrentVersion() -> impl IntoView {
    view! {
        <>
            <p class="text-neutral text-sm">Version: 0.0.1</p>
        </>

    }
}

#[component]
fn update() -> impl IntoView {
    view! {
        <>
            <button class="bg-blue-500 text-white px-6 py-2 rounded-md disabled:bg-gray-300 disabled:text-black" >Check for Update</button>
        </>

    }
}

#[component]
fn Copyright() -> impl IntoView {
    view! {
        <>
            <p class="text-neutral text-sm">Copyright 2024 Locale.ly<br/> All Rights Reserved</p>
        </>

    }
}
