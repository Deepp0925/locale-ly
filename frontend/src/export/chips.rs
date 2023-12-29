use leptos::*;

#[component]
pub fn Chips() -> impl IntoView {
    let formats = || {
        vec![
            ChipInfo {
                label: ".strings",
                selected: false,
            },
            ChipInfo {
                label: ".json",
                selected: false,
            },
            ChipInfo {
                label: ".yaml",
                selected: false,
            },
        ]
    };

    view! {
        <div class="flex flex-row flex-wrap">
            <For
                each=formats
                key=|item| item.label
                children=|item| {
                    view! {
                        <Chip info=RwSignal::new(item)/>
                    }
                }
            />
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct ChipInfo {
    label: &'static str,
    selected: bool,
}

impl ChipInfo {
    fn toggle(&mut self) {
        self.selected = !self.selected;
    }
}

#[component]
fn Chip(info: RwSignal<ChipInfo>) -> impl IntoView {
    let class = move || {
        let c = if info.get().selected {
            "bg-blue-500 text-white"
        } else {
            "bg-primary text-primary"
        };

        format!("rounded-lg mr-4 px-10 py-4 font-bold cursor-pointer {c}")
    };

    let toggle = move |_| info.update(|info| info.toggle());
    view! {
        <button class=class on:click=toggle>
            {info.get().label}
        </button>
    }
}
