use yew::prelude::*;
use shared::id::Id;
use yew_router::prelude::*;

use crate::components::FileData;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/files/:id")]
    File { id: Id },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<span>{"We are at home!"}</span>},
        Route::File { id } => html! { <FileData id = { id }/>},
        Route::NotFound => html! { <span>{ "404" }</span> },
    }
}
