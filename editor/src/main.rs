extern crate web_sys;
use shared::schema::*;
use std::cell::RefCell;
use std::rc::Rc;
use app::Editor;
mod app;
pub(crate) mod app_components;
pub(crate) mod components;
mod render;
mod plugins;
mod backend;
mod utils;
use yew::*;
use crate::app::EditorChange;


fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        // log!(&e)
    })
}


#[function_component]
pub fn App() -> Html {
//     let tree = Tree {
//         vertices: (),
//         adjacency: (),
//         id: Uuid::new_v4().into(),
//         elements: Elements {
//             root: Some(Uuid::new_v4().into()),
//             elements: vec![Element {
//                 id: Uuid::new_v4().into(),
//                 text: "Hello World".into(),
//                 attrs: HashMap::new(),
//                 children: vec![Element {
//                     id: Uuid::new_v4().into(),
//                     text: "Hello World".into(),
//                     attrs: HashMap::new(),
//                     children: vec![],
//                 }],
//             }],
//         },
//         root: None,
//     };
// };
// let element_tree = Rc::new(RefCell::new(tree.clone()));


html! {
    < >
        <h1>{"Text editor test"}</h1>
        // <Editor
        // title = {"untitled".to_string()}
        // element_tree={element_tree.clone()}
        // onchange = { onchange(element_tree.clone())}
        // />
    < / >
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}