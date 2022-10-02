use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::FileData;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/files/:id")]
    File{ id : Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<span>{"home"}</span>},
        Route::File {id} => html!{ <FileData id = { *id }/>},
        Route::NotFound => html! { <span>{ "404" }</span> },
    }
}
