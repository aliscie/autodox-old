use yew::prelude::*;
use yewdux::prelude::*;
use crate::utils::FileMap;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub id : u64
}

#[function_component(FileData)]
pub fn file_data(props : &Props) -> Html{
    let (data, dispatch) = use_store::<FileMap>();
    if data.data.get(&props.id).is_none() {
        dispatch.reduce_mut(|d| d.data.insert(props.id, "".to_string()));
    }
    html!{
        <>
        { data.data.get(&props.id).unwrap() }
        </>
    }
}
