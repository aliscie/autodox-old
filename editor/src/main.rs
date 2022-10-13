extern crate web_sys;
extern crate yew;

pub use app::Editor;
use yew::prelude::*;

mod app;

mod utils;
mod plugins;
mod element_tree;
pub(crate) mod app_components;
pub(crate) mod components;

use yew::prelude::*;
use yew::{function_component, html, Html};
#[function_component(App)]
pub fn app_component() -> Html {
    html! { <Editor title="text"/> }
}

fn main() {
    yew::start_app::<App>();
}
