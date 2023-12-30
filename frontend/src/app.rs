use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{dock::Dock, translations::Translations};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="h-full w-full bg-primary flex flex-col">
            <Translations />
            <Dock/>
        </div>
    }
}
