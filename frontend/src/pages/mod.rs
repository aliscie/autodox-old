use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

mod profile_info;
mod market;
mod settings;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub name: String,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PagesRoute {
    #[at("/page/profile")]
    Profile,
    #[at("/page/settings")]
    Settings,
    #[at("/page/market")]
    Market,
    #[not_found]
    #[at("/page/404")]
    NotFound,
}


pub fn page_switch(route: PagesRoute) -> Html {
    match route {
        PagesRoute::Profile => html! { <profile_info::ProfileInfo/> },
        PagesRoute::Market => html! { <market::Market/> },
        PagesRoute::Settings => html! { <settings::Settings/> },
        PagesRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>}
    }
}

