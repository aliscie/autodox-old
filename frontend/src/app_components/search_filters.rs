use yew::prelude::*;
use crate::components::{Menu, Avatar};
use web_sys::console::log_1;

#[function_component(SearchFiltes)]
pub fn search_filters() -> Html {

    let position: UseStateHandle<String> = use_state(|| "".to_string());

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(
            format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
        );
    });

    let items: Vec<Html> = vec![
        html! {<>{"Filter by name"}</>},
        html! {<>{"Filter by content"}</>},
        html! {<>{"Filter by type"}</>},
        html! {<>{"Filter by permission"}</>},
    ];

    html! { <>
    <Menu
    position={position.clone()}
     {items}
      />
      
      
    <div class="search_button">
       <div class="search">
          <input type="text" class="searchTerm" placeholder="Search..."/>

          <span class="searchButton btn">
            {"*"}
         </span>

         <span class="searchButton btn">
            {"A"}
         </span>

          <span {onmouseup} type="submit" class="right_clickable btn searchButton">
            <i class="fa fa-search"></i>
         </span>



       </div>
    </div>
        
    </>}
}
