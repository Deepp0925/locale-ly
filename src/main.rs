mod app;
mod components;
mod dock;
mod menu;
mod notifications;
mod settings;
mod translations;
use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
