use utils::alert;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

mod backend;
mod components;
mod extensions;
mod test;
mod utils;
mod router;

#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use components::{TitleBar, TitleBarButton};
use router::*;

use crate::components::TreeList;
use web_sys::{window, Document, Element, MouseEvent};

mod app;

use app::App;

fn main() {
    yew::start_app::<App>();
}

