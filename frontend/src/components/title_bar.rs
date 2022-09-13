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

    <div class="wrapper">
        <div class="header__container">
           <nav>
              <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/a/ab/Icon-Mac.svg" />
              <ul>
                {props.children.clone()}
              </ul>
           </nav>
           <div class="right-container">
              {props.right_content.clone()}
           </div>
        </div>
    </div>
        // <div
        // style={props.style.clone()}
        // data-tauri-drag-region="true" class="custom_title_bar">
        //     {props.children.clone()}
        //     <span style="margin-left:5%">{props.title.clone()}</span>
        // </div>
    };
}
