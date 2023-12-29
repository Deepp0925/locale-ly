mod chips;
mod state;
use std::time::Duration;

use chips::Chips;
use leptos::*;
pub use state::*;

#[component]
pub fn Export() -> impl IntoView {
    let state = use_export_read_signal();
    let is_open = MaybeSignal::derive(move || {
        state.get().unwrap_or(ExportState::Closed) == ExportState::Open
    });

    let close_export = |_| close_export();

    view! {
        <AnimatedShow
            when=is_open
            hide_delay=Duration::from_millis(0)
        >
            <div class="overlay h-full w-full z-10 fixed" on:click=close_export></div>
        </AnimatedShow>
        <AnimatedShow
            when=is_open
            hide_delay=Duration::from_millis(0)
        >
            <div class="export w-full fixed bottom-0 h-auto bg-secondary z-20 flex flex-col px-4 py-10 slide-in-up">
                <div class="flex justify-between items-center py-2">
                    <h1 class="font-bold text-xl text-primary">Export</h1>
                    <button class="ml-auto" on:click=close_export>
                        <i class="feather-x text-primary text-lg"></i>
                    </button>
                </div>
                <div class="h-5"></div>
                <Chips />
                <div class="h-8"></div>
                <div class="flex items-center">
                    <span class="text-primary text-sm mr-4">Output Folder:</span>
                    <Location />
                    <ExportBtn />
                </div>
            </div>
        </AnimatedShow>
    }
}

#[component]
fn location() -> impl IntoView {
    view! {
        <div class="flex items-center flex-1 mr-10">
            <input
                class="bg-primary text-primary rounded-lg px-4 py-4 mr-4 flex-1"
            />
            <button>
                <i class="feather-folder text-primary"></i>
            </button>
        </div>
    }
}

#[component]
fn export_btn() -> impl IntoView {
    let (state, set_state) = create_signal(false);

    let txt = move || {
        if state.get() {
            "Override Files"
        } else {
            "Export"
        }
    };

    let toggle = move |_| {
        set_state.set(!state.get_untracked());
    };

    let class = move || {
        let c = if state.get() {
            "bg-amber-500 text-white"
        } else {
            "bg-primary-opposite text-primary-opposite "
        };

        format!("rounded-lg px-20 py-4 duration-200 {c}")
    };

    view! {
        <button class=class on:click=toggle>
            <span class="font-bold">{txt}</span>
        </button>
    }
}
