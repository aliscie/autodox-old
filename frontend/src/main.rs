use yew::prelude::*;
mod backend;
mod components;
mod extensions;
mod test;
mod utils;

mod app;

use app::App;

fn main() {
    yew::start_app::<App>();
}