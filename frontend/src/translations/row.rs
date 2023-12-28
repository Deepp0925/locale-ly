use leptos::*;

use super::cell::{Cell, HeaderCell};

#[component]
pub fn Row() -> impl IntoView {
    let key = "hello_name";
    let src = "Hello, {name}!";
    let translations = [
        "Hola, {name}!",
        "Bonjour, {name}!",
        "Hello {}!",
        "Hello {name}!",
    ];
    view! {
        <tr class="px-3 items-start mb-4">
            <HeaderCell text={key.to_owned()}/>
            <Cell text={src.to_owned()} />
            <For
                each=move ||translations
                key = |translation|translation.to_owned()
                children=|translation| {
                    view! {
                        <Cell text={translation.to_owned()} />
                    }
                }
            />
        </tr>
    }
}
