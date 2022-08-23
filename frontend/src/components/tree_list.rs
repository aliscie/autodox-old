use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use web_sys::console::log_1;
use std::collections::HashMap;

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
