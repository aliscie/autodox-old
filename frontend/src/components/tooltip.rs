use web_sys::Element;
use yew::prelude::*;
use crate::shared::*;

#[derive(PartialEq, Properties)]
pub struct ToolTipProps {
    pub children: Children,
    pub context: Html,
    pub hover: Option<bool>,
    pub show: Option<bool>,
}

#[function_component]
pub fn ToolTip(props: &ToolTipProps) -> Html {
    let show = props.show;
    let hover = props.hover.unwrap_or(false);

    let popover = NodeRef::default();
    let display: UseStateHandle<bool> = use_state(|| show.unwrap_or(false));


    let props = props.clone();
    let mut style = "display: none".to_string();
    if (*display && show.is_none()) || show.unwrap_or(false) {
        let element = popover.cast::<Element>();
        style = format!("display: block; ").to_string();
        // if let Some(e) = element {
        //     let t = e.get_bounding_client_rect();
        //     style = format!("display: block; bottom:{}px; left:{}px", t.top(), t.left()).to_string();
        // }
    }
    let _hover = hover.clone();
    html! {
        <span
        ref={popover}
            onmouseleave={let display = display.clone();Callback::from(move |_| {   display.set(false); })}
            class={css_file_macro!("tooltip.css")}
        >
            <span class="tooltip-context"  {style}>{props.context.clone()}</span>
            <span

                onmouseenter={let display = display.clone();Callback::from(move |_| {   display.set(true); })}
                onmouseleave={let display = display.clone();Callback::from(move |_| {   display.set(_hover); })}
                >
                {props.children.clone()}
            </span>
        </span>
    }
}