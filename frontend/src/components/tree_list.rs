use crate::utils::{ FileNode, FileMap};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Element, MouseEvent};
use yew::prelude::*;
use yewdux::prelude::*;


pub trait MyNewTrait {
    fn target_element<'a>(&self) -> Option<Element>;
}

impl MyNewTrait for MouseEvent {
    fn target_element(&self) -> Option<Element> {
        let doc = window().unwrap_throw().document().unwrap_throw();
        let x = self.page_x();
        let y = self.page_y();
        doc.element_from_point(x as f32, y as f32)
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub children: Children,
    // pub files: Vec<HashMap>,
}

#[function_component(TreeList)]
pub fn tree_list() -> Html {
    let (root, dispatch) = use_store::<FileNode>();
    let d = Dispatch::<FileMap>::new();
    d.reduce_mut(|r| r.data.insert(234, "file one!".into()));
    d.reduce_mut(|r| r.data.insert(235, "file two!".into()));
    d.reduce_mut(|r| r.data.insert(224, "file three!".into()));
    d.reduce_mut(|r| r.data.insert(225, "file four!".into()));
    //let mut root = FileNode::new(0, "root".into());
    // later we will get these data from some external api as json
    let some_data = r#"
     [
         {
             "name":"filename",
             "id":234,
             "children":[
                 {
                 "name":"filename4",
                 "id":235
                 }
             ]
         },
         {
             "name":"filename2",
             "id":224
         },
         {
             "name":"filename3",
             "id":225
         }
     ]
        "#;
    //root.children = serde_json::from_str(some_data).unwrap();
    dispatch.reduce_mut(move |r| r.children = serde_json::from_str(some_data).unwrap());
    (&root.children).into_iter().map(|file| file.to_html()).collect::<Html>()
}
