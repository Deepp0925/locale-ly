use leptos::*;

use crate::settings::Settings;

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <div class="overlay h-full w-full fade-in z-10 fixed"></div>
        <div class="menu h-full fixed left-0 w-144 bg-secondary slide-in-left z-20 flex">
            <div class="my-8 mx-6 flex flex-col w-full">
                <div class="titlebar flex items-center justify-between">
                    <h1 class="font-bold text-3xl text-primary">Locale.ly</h1>
                    <button>
                        <i class="feather-x text-primary text-lg"></i>
                    </button>
                </div>

                <div class="flex flex-1 mt-4">
                    <div class="projects w-80">App the projects will be here</div>
                    <div class="divider my-16 bg-green-400 mx-6 mr-8 bg-neutral"></div>
                    <div class="settings flex-1 flex flex-col">
                        <Settings/>
                    </div>
                </div>

            </div>

        </div>

    }
}
