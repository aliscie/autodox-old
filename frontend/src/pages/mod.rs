use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;
mod market;
mod profile_info;
mod settings;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[derive(Clone, Routable, PartialEq)]
pub enum PagesRoute {
    #[at("/profile")]
    Profile,
    #[at("/settings")]
    Settings,
    #[at("/market")]
    Market,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn page_switch(route: PagesRoute) -> Html {
    match route {
        PagesRoute::Profile => html! { <profile_info::ProfileInfo/> },
        PagesRoute::Market => html! { <market::Market/> },
        PagesRoute::Settings => html! { <settings::Settings/> },
        PagesRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}
