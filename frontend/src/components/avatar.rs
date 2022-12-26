use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: Option<String>,
}

#[function_component]
pub fn Avatar(props: &Props) -> Html {
    let src = props.src.clone();
    let response: Html = match src {
        None => html! {<>
            // <span class="active-icon"></span>
            <i class="fas fa-user"></i>
       </>
        },
        Some(link) => html! {<>
            // <span class="active-icon"></span>
            <img src={link} alt="Avatar" class="avatar"/>
       </>}
    };
    response
}
