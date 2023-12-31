use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{dock::Dock, project::ProjectState, translations::Translations};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// #[component]
// pub fn App() -> impl IntoView {
//     // initialize global state
//     provide_context(create_rw_signal(ProjectState::new()));

//     view! {
//         <div class="h-full w-full bg-primary flex flex-col">
//             <Translations />
//             <Dock/>
//         </div>
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    let args = GreetArgs { name: "World" };
    let result = invoke("greet", JsValue::symbol(todo!()));

    view! {
        <div class="h-full w-full bg-primary flex flex-col">

        </div>
    }
}
