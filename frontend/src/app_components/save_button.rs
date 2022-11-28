use web_sys::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

use shared::schema::SaveButtonState;

use crate::*;

// use shared::schema::EditorElementUpdate;

// #[derive(PartialEq, Properties)]
// pub struct SaveButtonProps {
//     // pub id: u64,
// }


#[function_component(SaveButton)]
pub fn save_button(
    // props: &SaveButtonProps
) -> Html {
    let (state, dispatch) = use_store::<SaveButtonState>();

    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        // TODO
        //  let is_saved  crate::backend::update_file(state.element.id, state.data);
        //  if is_saved{
        dispatch.reduce_mut(|state| state.is_saved = true);
        //   }
    });
    let mut style = "color: tomato";
    let mut class = "fa-solid fa-x";
    if state.is_saved {
        style = "color: lightgreen; transition: 0.2s";
        class = "fa-solid fa-check";
    }

    let mut res = html! {""};
    if *IS_WEB {
        res = html! {<>
            <span  {onmouseup} class="btn" ><i
            {style}
            {class}></i>{"Save"}</span>
        </>};
    };
    res
}
