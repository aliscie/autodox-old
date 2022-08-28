extern crate web_sys;
use yew::prelude::*;


#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div contenteditable="true">
        {"text editor"}
        </div>

    }
}

fn main() {
    yew::start_app::<Editor>();
}