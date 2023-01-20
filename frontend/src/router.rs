use crate::pages::*;
use crate::specific_components::FileData;
use shared::id::Id;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/*")]
    Page,
    #[at("/files/:id")]
    File { id: Id },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    let fallback = html! { <div> {"loading"}</div>};
    match routes {
        // Dashboard
        Route::Home => html! {<span>{"We are at home!"}</span>},
        Route::File { id } => html! { <Suspense {fallback}><FileData id = {id}/></Suspense>},
        Route::NotFound => html! { <span>{"404"}</span> },
        Route::Page => html! { <Switch<PagesRoute> render={page_switch}/>},
    }
}
