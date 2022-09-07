use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: Html,
    pub children: Children,
}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    return html! {
        <div data-tauri-drag-region="true" class="custom_title_bar">
           <div class="buttons">
                {props.children.clone()}
            </div>
            <span style="margin-left:5%">{props.title.clone()}</span>
        </div>
    };
}
