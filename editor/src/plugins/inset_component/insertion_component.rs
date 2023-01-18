use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{Element, KeyboardEvent, MouseEvent, Node, window};
use yew::prelude::*;

use shared::*;
use crate::plugins::inset_component::types::{CommandItems, DropDownItem, Position};
use crate::plugins::inset_component::utiles;

#[wasm_bindgen(module = "/src/plugins/inset_component/caret_position.js")]
extern "C" {
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position() -> u32;
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub trigger: String,
    pub items: CommandItems,
}


#[function_component]
pub fn EditorInsert(props: &Props) -> Html {
    let position: UseStateHandle<Option<Position>> = use_state(|| None);
    let input_text: UseStateHandle<String> = use_state(|| "".to_string());
    let _input_text = input_text.clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let text_editor = doc.query_selector(".text_editor").unwrap().unwrap();
    let trigger = props.trigger.clone();
    let items = props.items.clone();
    let _trigger = trigger.clone();
    let _position = position.clone();

    use_effect_with_deps(
        move |editor_ref| {
            utiles::trigger_popover(&text_editor, _trigger, _position, _input_text);
        },
        (),
    );
    let _position = position.clone();

    if (*position.clone()).is_none() {
        return html! {
        <></>
    };
    };
    let p = (&*_position).as_ref().unwrap();


    html! {
            < span class ={css_file_macro ! ("dropdown.css")} >

            < span id = "editor_dropdown" style ={format ! (" top:{}px; left:{}px", p.y, p.x)}>

            {
            items
            // .iter()
            // .sort_by( | a,&element | element.contains( & * input_text.replace(" ", "").replace("/", "")))
            // .cloned()
            .into_iter().map( | item | {
            html !{<a>{item.value}< / a >}
            }).collect::< Html > ()
            }

            < / span >
            < / span >
            }
}