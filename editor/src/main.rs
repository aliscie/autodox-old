extern crate web_sys;
extern crate yew;
pub use app::Editor;
mod app;
pub(crate) mod app_components;
pub(crate) mod components;
mod render;
mod plugins;
mod utils;
use yew::{function_component, html};

#[function_component(App)]
pub fn app_component() -> Html {
    html! { <Editor title="text"/> }
}

fn main() {
    yew::start_app::<App>();
}
