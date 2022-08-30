mod backend;
mod components;
mod extensions;
mod test;
mod utils;
mod router;
mod app;

use app::App;

fn main() {
    yew::start_app::<App>();
}

