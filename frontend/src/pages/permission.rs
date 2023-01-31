use crate::backend;
use crate::shared::*;
use shared::{
    id::Id,
    schema::{FileDirectory, FileMode, FileNode},
};
use std::pin::Pin;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::*;
use yewdux::prelude::Dispatch;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_id: Id,
}

#[function_component]
pub fn Permission(props: &Props) -> Html {
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    let file_node_state: UseStateHandle<Option<FileNode>> = use_state(|| None);

    let _file_id = props.file_id.clone();
    let _file_node_state = file_node_state.clone();
    use_future(move || async move {
        let res = backend::get_file(_file_id).await;
        log!(&res);
        if let Ok(_file_node) = res {
            _file_node_state.set(Some(_file_node));
        }
    });

    let mut is_public = false;
    let mut is_private = false;
    let mut is_restricted = false;
    if let Some(_file_node) = (*file_node_state).clone() {
        let _file_mode = _file_node.file_mode.clone();
        is_public = _file_mode == FileMode::Public;
        is_private = _file_mode == FileMode::Private;
        is_restricted = _file_mode == FileMode::Restricted;
    }

    let _file_node_state = file_node_state.clone();
    let on_radio_change = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();
        let permission = input.value();
        let mut _file_node = (*_file_node_state).clone().unwrap_or_default();
        if permission == "Private".to_string() {
            _file_node.file_mode = FileMode::Private;
        }
        if permission == "Restricted".to_string() {
            _file_node.file_mode = FileMode::Restricted;
        }
        if permission == "Public".to_string() {
            _file_node.file_mode = FileMode::Public;
        }
        _file_node_state.set(Some(_file_node));
    });

    let _file_node_state = file_node_state.clone();
    let on_save: Callback<MouseEvent> =
        dispatch_file_directory.reduce_mut_future_callback_with(move |state, _e: MouseEvent| {
            let _file_node_state = _file_node_state.clone();
            Box::pin(async move {
                let _file_node = (*_file_node_state).clone().unwrap_or_default();
                let res = backend::update_file(_file_node).await;
                // log!(&res);
            })
        });

    html! {
        <div class="m-8 flex flex-col gap-8 items-center justify-center">
            <div class="flex flex-row gap-8">
                <div>
                    <input type="radio" id="private" name="permission" value="Private" checked={is_private} onchange={&on_radio_change}/>
                    <label class="cursor-pointer" for="private">{"Private"}</label>
                </div>
                <div>
                    <input type="radio" id="restricted" name="permission" value="Restricted" checked={is_restricted} onchange={&on_radio_change}/>
                    <label class="cursor-pointer" for="restricted">{"Restricted"}</label>
                </div>
                <div>
                    <input type="radio" id="public" name="permission" value="Public" checked={is_public} onchange={&on_radio_change}/>
                    <label class="cursor-pointer" for="public">{"Public"}</label>
                </div>
            </div>
            <button class="px-4" onclick={on_save}>{"Save"}</button>
        </div>
    }
}
