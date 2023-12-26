mod locale;
mod theme;
mod version;
use leptos::*;

use locale::Locale;
use theme::Theme;
use version::Version;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <>
            <h2 class="font-semibold text-lg text-primary">Settings</h2>
            <Locale/>
            <Theme/>
            <Version/>
        </>

    }
}
