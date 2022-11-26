use web_sys::{MouseEvent};
use yew::prelude::*;
use shared::log;
use yewdux::prelude::use_store;
use shared::schema::FileDirectory;
use crate::*;

#[derive(PartialEq, Properties)]
pub struct SaveButtonProps {
    // pub id: u64,
}

#[function_component(SaveButton)]
pub fn save_button(props: &SaveButtonProps) -> Html {
    let (file_tree, _) = use_store::<FileDirectory>();
    let style: UseStateHandle<&str> = use_state(|| "color:lightgreen");
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        // TODO
        //  crate::backend::update_file(file_tree.current_file_id, yewdux::get("file_new_data"));
        log!("save...");
    });

    // props.on_content_change(|_|=>{
    //     style.set("color:tomato");
    // });

    let mut res = html! {""};
    if *IS_WEB {
        res = html! {<>
            <span  {onmouseup} class="btn" ><i style={*style} class="fa-solid fa-check"></i>{"Save"}</span>
        </>};
    };
    res
}
