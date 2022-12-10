use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

use shared::log;
use shared::schema::SaveButtonState;

use crate::*;

// use shared::schema::EditorElementUpdate;

#[derive(PartialEq, Properties)]
pub struct SaveButtonProps {
    // pub id: u64,
}


#[function_component(SaveButton)]
pub fn save_button(props: &SaveButtonProps) -> Html {
    let (state, dispatch) = use_store::<SaveButtonState>();
    let button_state = Dispatch::<SaveButtonState>::new();

    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        // TODO
        //  let is_saved  crate::backend::update_file(state.element.id, state.data);
        //  if is_saved{
        button_state.set(SaveButtonState { is_saved: true });
        log!("save...");
        //   }
    });


    let mut res = html! {""};
    if *IS_WEB {
        res = html! {<>
            <span  {onmouseup} class="btn" ><i
            style={format!("{:?}",&state.style)}
            class="fa-solid fa-check"></i>{"Save"}</span>
        </>};
    };
    res
}
