extern crate web_sys;

use yew::prelude::*;

// this is used for the work space

#[function_component(Editor)]
pub fn editor() -> Html {
    // TODO
    // get mouse position and sort it in yewdux
    // each time the mouse move sort the pagex and pagey again

    // get current hovered element and sort it yewdux
    // get the previous  hovered and sorted it in yewdux

    // get the current focused and sorted it
    // get the previous  focused and sorted it in yewdux

    html! {
        <div contenteditable="true">
        {"text editor"}
        </div>
    }
}
