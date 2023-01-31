use std::cmp::Ordering;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{Element, KeyboardEvent, MouseEvent, Node, Range, window};
use yew::prelude::*;
use yew::suspense::{SuspensionResult, UseFutureHandle};
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
    pub command: fn(DropDownItem, Option<Range>),
}


#[function_component]
pub fn EditorInsert(props: &Props) -> Html {

    let trigger = props.trigger.clone();
    let items = props.items.clone();

    let items: UseStateHandle<Vec<DropDownItem>> = use_state(|| items);
    let command = props.command.clone();

    let _trigger = trigger.clone();
    let _items = items.clone();
    let handle_command: Callback<Range> = Callback::from(move |range| {
        command(_items[0].clone(), Some(range));
    });

    let (input_text,position ) = utiles::use_trigger_popover(trigger.clone(), handle_command);
    let _input_text = input_text.clone();

    let _items = items.clone();
    let mut sorted_items = (&*items).clone();
    let _items = items.clone();
    let _input_text = input_text.clone();
    let _trigger = trigger.clone();
    let x = "done";


    use_effect_with_deps(
        move |editor_ref| {
            sorted_items.sort_by(|a, b| {
                let a = a.text.to_lowercase();
                let b = b.text.to_lowercase();
                let input_text = (*_input_text).to_lowercase().replace(" ", "").replace(_trigger.as_str(), "");
                let a = a.starts_with(&input_text);
                let b = b.starts_with(&input_text);
                if a && b {
                    return a.cmp(&b);
                }
                if a {
                    return std::cmp::Ordering::Less;
                }
                if b {
                    return std::cmp::Ordering::Greater;
                }
                std::cmp::Ordering::Equal
            });
            _items.set(sorted_items.clone());
        },
        input_text.clone(),
    );

    if (*position.clone()).is_none() {
        return html! {
        <></>
    };
    };
    let p = (&*position).as_ref().unwrap();
    let items = items.clone();


    html! {
            <span class ={css_file_macro ! ("dropdown.css")}>

            <select size="5" id = "editor_dropdown" style ={format ! (" top:{}px; left:{}px", p.y, p.x)}>
            {
            (&*items).clone()
            .into_iter().map( | item | {
                let _item = item.clone();
                html !{<option >{item.value}</ option>}
            }).collect::<Html> ()
            }

            </ select>
            </ span>
            }
}