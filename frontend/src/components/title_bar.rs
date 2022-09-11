use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: Html,
    pub children: Children,
    pub style: String,
}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    return html! {
        <div
        style={props.style.clone()}
        data-tauri-drag-region="true" class="custom_title_bar">
            {props.children.clone()}
            <span style="margin-left:5%">{props.title.clone()}</span>
        </div>
    };
}
