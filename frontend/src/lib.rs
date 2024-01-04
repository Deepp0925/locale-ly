mod app;
mod components;
mod dock;
mod export;
mod locales;
mod menu;
mod notifications;
mod project;
mod settings;
mod translations;
mod utils;
// use app::*;
use leptos::*;
mod appr;
use appr::*;

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
