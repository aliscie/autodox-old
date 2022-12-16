use yew::prelude::*;

use crate::components::{Avatar, ContextMenu};

#[function_component(SearchFiltes)]
pub fn search_filters() -> Html {
    let event: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let replace: UseStateHandle<bool> = use_state(|| false);

    let _event = event.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _event.set(Some(_e));
    });

    let _replace = replace.clone();
    let onmousedown: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if *_replace == false {
            _replace.set(true);
        } else {
            _replace.set(false);
        }
    });

    let items: Vec<Html> = vec![
        html! {<a>{"Filter by name"}</a>},
        html! {<a>{"Filter by content"}</a>},
        html! {<a>{"Filter by category"}</a>},
        html! {<a>{"Filter by tag"}</a>},
        html! {<a>{"Filter by permission"}</a>},
        html! {<a {onmousedown} >{"Replace"}</a>},
    ];

    html! { <>
    <div class="search_button">
       <div class="search">
          <input type="text" class="searchTerm" placeholder="Search..."/>
          <input
          style={format!("{}", if (*replace).clone() { "display: block"} else {"display: none"})}
          type="text" class="searchTerm" placeholder="Replace..."/>


          <span class="searchButton btn">
            {"*"}
         </span>

         <span class="searchButton btn">
            {"A"}
         </span>

          <span {onmouseup} class="right_clickable btn searchButton">
            <i class="fa fa-search"></i>
         </span>



       </div>
    </div>

    </>}
}
