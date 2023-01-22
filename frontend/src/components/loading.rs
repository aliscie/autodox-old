use yew::prelude::*;
use shared::*;

#[derive(PartialEq, Properties)]
pub struct LoadingProps {
    // pub id: u64,
}

#[function_component]
pub fn Loading(props: &LoadingProps) -> Html {
    return html! {<span
        class={css_file_macro!("loading.css")}
        >
            <div class="loading">
            <span></span>
            <span></span>
            <span></span>
            </div>
    </span>};
}
