use yew::prelude::*;
use yewdux::prelude::*;
use crate::utils::FileTree;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub id : u64
}

#[function_component(FileData)]
pub fn file_data(props : &Props) -> Html{
    let dispatch = Dispatch::<FileTree>::new();
    html!{
        <>
        { &dispatch.get().vertices.get(&props.id).unwrap().data }
        </>
    }
}
