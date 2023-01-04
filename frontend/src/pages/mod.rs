use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

mod profile_info;
mod market;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub name: String,
}

#[derive(Clone, Routable, PartialEq)]
pub enum PagesRoute {
    #[at("/page/profile")]
    Profile,
    #[at("/page/market")]
    Market,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}


pub fn PageSwitch(route: PagesRoute) -> Html {
    match route {
        PagesRoute::Profile => html! { <profile_info::ProfileInfo/> },
        PagesRoute::Market => html! { <market::Market/> },
        PagesRoute::NotFound => html! {<span>{"xx"}</span>}
        // PagesRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

