use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: Option<String>,
    pub size: Option<i32>,
}

#[function_component]
pub fn Avatar(props: &Props) -> Html {
    let size = props.size;;
    let src = props.src.clone();
    let mut style= "".to_string();
    if let Some(size) = size {
        style = format!("width:{}px;height:{}px;", size, size).to_string();
    }
    let response: Html = match src {
        None => html! {<>
            // <span class="active-icon"></span>
            <i class="fas fa-user"></i>
        </>},
        Some(link) => html! {<>
            // <span class="active-icon"></span>
            <img {style} src={link} alt="Avatar" class="avatar"/>
        </>},
    };
    response
}
