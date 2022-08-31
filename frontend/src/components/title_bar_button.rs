use yew::prelude::*;
use web_sys::MouseEvent;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub button_type: String,
    pub children: Children,
    pub onclick: Option<Callback<MouseEvent>>
}

#[function_component(TitleBarButton)]
pub fn title_bar_button(props: &Props) -> Html {
    html! {
        <div
        onclick={props.onclick.clone()}
         class={format!("barr_button {}",props.button_type)}
         >
        <a class="barr_button_content" href="#"><span>{props.children.clone()}</span></a>
        </div>
    }
}
