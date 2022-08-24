use crate::utils::FileNode;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Element, MouseEvent, console};
use yew::prelude::*;

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

// data structure
// [
//     {
//         "name":"filename",
//         "id":234,
//         "children":[
//             {
//             "name":"filename",
//             "id":234,
//             }
//
//         ]
//     },
//     {
//         "name":"filename2",
//         "id":224,
//         ]
//     },
//     {
//         "name":"filename3",
//         "id":224,
//         ]
//     },
// ]

#[function_component(TreeList)]
pub fn tree_list() -> Html {
    let mut root = FileNode::new(0, "root".into());
    // later we will get these data from some external api as json
    let some_data = r#"
     [
         {
             "name":"filename",
             "id":234,
             "children":[
                 {
                 "name":"filename",
                 "id":234
                 }
             ]
         },
         {
             "name":"filename2",
             "id":224
         },
         {
             "name":"filename3",
             "id":224
         }
     ]
        "#;
    root.children = serde_json::from_str(some_data).unwrap();
    //console::log_1(serde_json::to_string(&root).into());
    html! {

                    //  {
                    //     props.files.into_iter().map(|file| {

                    //         <li >{file["name"]} <span style="background: gray;">{":"}</span> </li>
                    //         if file.contains_key("children") {<ul class="nested"><TreeList files={file["children"]}/></ul>}
                    //     }).collect::<Html>()
                    // }

                    <li><span class="caret">{"Beverages"}</span>
                    <ul class="nested">
                        <li >{"Water"} <span style="background: gray;">{":"}</span> </li>
                        <li >{"Coffee"}</li>
                        <li><span class="caret">{"Tea"}</span>
                        <ul class="nested">
                            <li>{"Black Tea"}</li>
                            <li>{"White Tea"}</li>
                            <li><span class="caret">{"Green Tea"}</span>
                            <ul class="nested">
                                <li>{"Sencha"}</li>
                                <li>{"Gyokuro"}</li>
                                <li>{"Matcha"}</li>
                                <li>{"Pi Lo Chun"}</li>
                            </ul>
                            </li>
                        </ul>
                        </li>
                    </ul>
                    </li>




    }
}
