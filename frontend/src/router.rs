use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::FileData;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/files/:id")]
    File{ id : u64 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<h1>{"home"}</h1>},
        Route::File {id} => html!{ <FileData id = { *id }/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
