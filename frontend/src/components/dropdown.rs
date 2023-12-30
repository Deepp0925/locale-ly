use leptos::*;

#[derive(Debug, Clone)]
pub struct DropDownItem {
    pub text: String,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
}

#[component]
pub fn DropDown(items: Vec<DropDownItem>) -> impl IntoView {
    view! {
        <div class="w-full max-w-sm flex justify-between items-center bg-primary p-3 rounded-md">
            <select class="select w-full border-none text-primary">
                <For
                    each=move || items.clone()
                    key=move |item| item.value.clone()
                    children=move |item| {
                        view! {
                            <option
                                value=&item.value
                                selected=item.selected
                                disabled=item.disabled
                            >
                                {&item.text}
                            </option>
                        }
                    }

                />
            </select>
            <label class="label"><i class="feather-chevron-down text-primary"></i></label>
        </div>
    }
}
