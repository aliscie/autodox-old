mod backend;
mod components;
mod app_components;
mod extensions;
mod test;
mod utils;
mod router;
mod app;

use app::App;

use lazy_static::lazy_static;



lazy_static! {
    pub static ref IS_WEB: bool = {
        let mut m : bool = false;
        #[cfg(feature = "web")] {
            m = true;
        }
        m
    };
}

fn main() {
    yew::start_app::<App>();
}

