use crate::utils::FileTree;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u64,
}

#[function_component(FileData)]
pub fn file_data(props: &Props) -> Html {
    let dispatch = Dispatch::<FileTree>::new();
    html! {
        <>
        { &dispatch.get().files.vertices.get(&props.id).unwrap().data }
        </>
    }
}
