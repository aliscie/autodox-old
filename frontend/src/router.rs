use yew::prelude::*;
use shared::id::Id;
use yew_router::prelude::*;

use crate::app_components::FileData;

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
    let fallback = html!{ <div> {"loading"}</div>};
    match routes {
        Route::Home => html! {<span>{"We are at home!"}</span>},
        Route::File { id } => html! { <Suspense {fallback}> <FileData id = { id }/> </Suspense>},
        Route::NotFound => html! { <span>{ "404" }</span> },
    }
}
