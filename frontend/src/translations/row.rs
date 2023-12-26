use leptos::*;

use super::cell::Cell;

#[component]
pub fn Row() -> impl IntoView {
    let key = "hello_name";
    let src = "Hello, {name}!";
    let translations = ["Hola, {name}!", "Bonjour, {name}!"];
    view! {<div class="flex">
        <Cell text={key.to_owned()}/>
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
    </div>}
}
