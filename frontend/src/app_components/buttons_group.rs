use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component(ButtonsGroup)]
pub fn buttons_group(props: &Props) -> Html {
    let onclick = Callback::from(move |e: MouseEvent| {});
    let handle_files = Callback::from(move |e: MouseEvent| {});

    html! {
        <div class="buttons_group_class">
            // {catigories.map((files),{
            //     html!{
            //         <span onclick={handle_files} class="btn">{file.name}</span>
            //     }
            // })}
            <span class="btn">{"work"}</span>
            <span class="btn">{"school"}</span>
            <span class="btn">{"projects"}</span>
            <span {onclick} class="btn"><i class="fa fa-plus"></i></span>
        </div>
    }
}
