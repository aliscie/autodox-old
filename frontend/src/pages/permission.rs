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
        <div class="w-full h-full">
            <div>
                <div class="text-red-500">{"Permission"}</div>
            </div>
        </div>
    }
}
