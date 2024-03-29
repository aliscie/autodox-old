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
    #[at("/files/:id/:auther")]
    File { id: Id, auther: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    let fallback = html! { <div class="switch loader"/>};
    match routes {
        // Dashboard
        Route::Home => html! {<span>{"We are at home!"}</span>},
        Route::File { id, auther } => html! {<Suspense {fallback}> <FileData {auther} {id}/></Suspense>},
        Route::NotFound => html! { <span>{"404"}</span> },
        Route::Page => html! { <Switch<PagesRoute> render={page_switch}/>},
    }
}
