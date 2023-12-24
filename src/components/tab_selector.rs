use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub disabled: bool,
}

impl From<String> for Tab {
    fn from(name: String) -> Self {
        Self {
            name,
            disabled: false,
        }
    }
}

impl From<&str> for Tab {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            disabled: false,
        }
    }
}

#[component]
pub fn TabSelector<const N: usize>(
    /// a unique id for the tab bar
    name: &'static str,
    /// active tab index
    active: usize,
    /// list of all the tabs
    tabs: [Tab; N],
    /// on change callback
    on_change: fn(usize),
) -> impl IntoView {
    let (checked, set_checked) = create_signal(active);

    let update_change = move |tab: usize| {
        set_checked.set(tab);
        on_change(tab);
    };
    let tabs = tabs.into_iter().enumerate();

    view! {
        <div class="flex">
            <div class="flex w-auto bg-primary p-2 rounded-xl tabs">
                <For
                    each=move || tabs.clone()
                    key= move|tab| tab.0
                    children=move |(i, tab)| {
                        let id = format!("{}-{}", name, i);
                        view! {
                            <input type="radio"
                                name=name
                                id=&id
                                disabled=tab.disabled
                                checked=move || i == checked.get()
                                on:change=move|_| update_change(i)
                            />
                            <label for=id class="tab w-28 h-10 rounded-xl text-primary font-medium">{tab.name}</label>
                        }
                    }
                />
                <span class="tab-selector bg-secondary w-28 h-10 rounded-lg shadow-lg"></span>
            </div>
        </div>
    }
}
