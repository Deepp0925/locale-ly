mod state;
use std::time::Duration;

use leptos::*;

#[component]
pub fn EditLocalesBtn() -> impl IntoView {
    let state = create_rw_signal(false);
    let open = move |_| state.set(true);

    view! {
        <button class="px-6" on:click=open>
            <p class="text-blue-500">Edit Locales</p>
        </button>
        <EditLocales state/>
    }
}

#[component]
fn EditLocales(state: RwSignal<bool>) -> impl IntoView {
    let close = move |_| state.set(false);

    view! {
        <Portal>
            <AnimatedShow
                when=state
                hide_delay=Duration::from_millis(0)
            >
                <div class="overlay fade-in" on:click=close></div>
            </AnimatedShow>
            <AnimatedShow
                when=state
                hide_delay=Duration::from_millis(0)
            >
                <div class="locales w-96 fixed top-0 right-0 h-full bg-secondary z-20 flex flex-col px-4 py-10 slide-in-right">
                    <div class="flex items-center">
                        <h1 class="font-bold text-xl text-primary">Edit Locales</h1>
                    </div>
                    <div class="h-5"></div>
                    <div class="flex flex-col flex-1">
                        <h3 class="text-primary font-bold">Source Locale</h3>
                        <p class="text-primary">English (en)</p>
                        <div class="h-5"></div>
                        <h3 class="text-primary font-bold">Target Locales</h3>
                        <Locale />
                    </div>
                    <div class="h-5"></div>
                    <button class="bg-primary text-primary rounded-lg w-full py-4" on:click=close>
                        close
                    </button>
                </div>
            </AnimatedShow>
        </Portal>
    }
}

#[component]
fn locale() -> impl IntoView {
    view! {
        <div class="flex items-center">
            <div class="checkbox-wrapper w-full">
                <label class="flex py-4">
                    <p>Spanish (es)</p>
                    <input type="checkbox"/>
                    <span class="checkbox border-text-primary border-2"></span>
                </label>
            </div>

            // <input
            //     class="bg-primary text-primary rounded-lg px-4 py-4 mr-4 flex-1"
            // />
            // <button>
            //     <i class="feather-trash text-red-500"></i>
            // </button>
        </div>
    }
}
