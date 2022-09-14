use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: Html,
    pub children: Children,
    pub style: String,
    pub right_content: Html,
}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    return html! {

    <div  data-tauri-drag-region="true" class=" custom_title_bar wrapper">
        <div style={props.style.clone()} class="header__container">
           <nav>
              <ul style="margin-top:10px;">
                {props.children.clone()}
                {props.title.clone()}
              </ul>
           </nav>

           <div class="right-container">
              {props.right_content.clone()}
           </div>
        </div>
    </div>
    };
}
