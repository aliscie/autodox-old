use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, MouseEvent, window, Document};
use web_sys::console::log_1;


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


#[function_component(TreeList)]
pub fn tree_list() -> Html {
    // let doc = window().unwrap_throw().document().unwrap_throw();
    // let body = &doc.query_selector("#myUL").unwrap().unwrap();
    // let body_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
    //     let curr = e.target_element().unwrap();
    //     curr.class_list().toggle("caret-down");
    //
    //     &curr.parent_element().unwrap().query_selector(".nested").unwrap().unwrap().class_list().toggle("active");
    // }) as Box<dyn FnMut(_)>);
    // &body.add_event_listener_with_callback("click", body_closure.as_ref().unchecked_ref()).unwrap_throw();
    // body_closure.forget();
    // draggable="true"


    // let on_mouse_over: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
    // });

    // let on_leave: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
    // });

    // let toggle_caret: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
    // });


    html! {
        <span>
                    <ul  id="myUL">
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
                </ul>



        </span>
    }
}
