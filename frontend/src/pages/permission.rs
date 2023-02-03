use crate::backend;
use crate::shared::*;
use shared::{id::Id, schema::FileNode};
use yew::prelude::*;
use yew::suspense::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_id: Id,
}

#[function_component]
pub fn Permission(props: &Props) -> Html {
    let file_node: UseStateHandle<Option<FileNode>> = use_state(|| None);

    let _file_node = file_node.clone();
    let _file_id = props.file_id.clone();
    use_future(move || async move {
        // TODO
        //  let res = backend::update_file(new_data,_file_id.clone()).await;
        //  On update file U can get file from yewdux
        //  then call update_file
        //  No need to call get_file at all, becuase the files the you own are all in yewdux come with get_directories
    });

    html! {
        <div class={css_file_macro!("permission.css")}>
            <div class="container">
                <div class="radio-group">
                    <div>
                        <input type="radio" id="public" name="permission" value="Public"/>
                        <label class="cursor-pointer" for="public">{"Public"}</label>
                    </div>
                    <div>
                        <input type="radio" id="private" name="permission" value="Private"/>
                        <label class="cursor-pointer" for="private">{"Private"}</label>
                    </div>
                    <div>
                        <input type="radio" id="restricted" name="permission" value="Restricted"/>
                        <label class="cursor-pointer" for="restricted">{"Restricted"}</label>
                    </div>
                </div>
                <button class="btn">{"Save"}</button>
            </div>
        </div>

    }
}
