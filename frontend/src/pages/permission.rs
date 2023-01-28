use crate::shared::*;
use shared::{id::Id, schema::FileNode};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_id: Option<Id>,
}

#[function_component]
pub fn Permission(props: &Props) -> Html {
    let file_node: UseStateHandle<Option<FileNode>> = use_state(|| None);

    html! {
        <div class="m-8 flex flex-col gap-8 items-center justify-center">
            <div class="flex flex-row gap-8">
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
        </div>
    }
}
