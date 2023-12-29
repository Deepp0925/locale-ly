mod app;
mod components;
mod dock;
mod export;
mod menu;
mod notifications;
mod settings;
mod translations;
use app::*;
use leptos::*;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("assets/locales", fallback = "en");

pub fn run_app() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
