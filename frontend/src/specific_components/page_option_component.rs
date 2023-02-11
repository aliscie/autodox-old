use crate::components::PopOverMenu;
use crate::router::Route;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(PageOptions)]
pub fn page_options() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let _position = position.clone();
    let on_click = Callback::from(move |e: MouseEvent| {
        crate::shared::log!("click");
        _position.set(Some(e));
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-comment"></i>{"Comments"}</a>},
        html! {<a><i class="fa-solid fa-rectangle-history"></i>{"History"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Share"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Export google docs"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Export dox"}</a>},
        html! {<a><i class="fa-solid fa-file-pdf"></i>{"Export pdf"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Delete"}</a>},
    ];

    html! {
        <>
            <PopOverMenu items={items} position={position.clone()}/>
            <span onclick={on_click} class="btn">
                <i class="fa-solid fa-ellipsis-vertical"></i>
            </span>
        </>
    }
}
