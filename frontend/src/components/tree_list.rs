use crate::utils::{FileNode, FileMap};
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
pub struct TreeListProps {
    pub title: Option<String>,
    pub children: Option<Children>,
    pub files: Vec<FileNode>,
}

#[function_component(TreeList)]
pub fn tree_list(props: &TreeListProps) -> Html {
    let (root, dispatch) = use_store::<FileNode>();
    let d = Dispatch::<FileMap>::new();
    d.reduce_mut(|r| r.data.insert(234, "file one!".into()));
    d.reduce_mut(|r| r.data.insert(235, "file two!".into()));
    d.reduce_mut(|r| r.data.insert(224, "file three!".into()));
    d.reduce_mut(|r| r.data.insert(225, "file four!".into()));

    (props.files.clone()).into_iter().map(|file| file.to_html()).collect::<Html>()
}