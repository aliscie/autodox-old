use shared::*;
use std::cmp::Ordering;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, Element, KeyboardEvent, MouseEvent, Node, Range};
use yew::prelude::*;
use yew::suspense::{SuspensionResult, UseFutureHandle};

use crate::plugins::inset_component::types::{CommandItems, DropDownItem, Position};
use crate::plugins::inset_component::utiles;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;


#[wasm_bindgen(module = "/src/plugins/inset_component/caret_position.js")]
extern "C" {
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position() -> u32;
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub trigger: String,
    pub items: CommandItems,
    pub command: Callback<(DropDownItem, Option<Range>)>,
}

#[function_component]
pub fn EditorInsert(props: &Props) -> Html {
    let trigger = props.trigger.clone();
    let items = props.items.clone();

    let items: UseStateHandle<Vec<DropDownItem>> = use_state(|| items);

    let _trigger = trigger.clone();
    let _items = items.clone();
    let command = props.command.clone();

    let handle_command: Callback<Range> = {
        let command = command.clone();
        Callback::from(move |range| {
            command.emit((_items[0].clone(), Some(range)));
        })
    };

    let (input_text, position) = utiles::use_trigger_popover(trigger.clone(), handle_command.clone());
    let _input_text = input_text.clone();

    let _items = items.clone();
    let mut sorted_items = (&*items).clone();
    let _items = items.clone();
    let _input_text = input_text.clone();
    let _trigger = trigger.clone();
    let x = "done";

    use_effect_with_deps(
        move |editor_ref| {
            // sorted_items.sort_by(|a, b| {
            //     let a = a.text.to_lowercase();
            //     let b = b.text.to_lowercase();
            //     let input_text = (*_input_text)
            //         .to_lowercase()
            //         .replace(" ", "")
            //         .replace(_trigger.as_str(), "");
            //     let a = a.starts_with(&input_text);
            //     let b = b.starts_with(&input_text);
            //     if a && b {
            //         return a.cmp(&b);
            //     }
            //     if a {
            //         return std::cmp::Ordering::Less;
            //     }
            //     if b {
            //         return std::cmp::Ordering::Greater;
            //     }
            //     std::cmp::Ordering::Equal
            // });

            let input_text = (*_input_text)
                .to_lowercase()
                .replace(" ", "")
                .replace(_trigger.as_str(), "");
            let matcher = SkimMatcherV2::default();
            sorted_items.sort_unstable_by(|a, b| {
                let a_distance = matcher.fuzzy_match(&a.text, &input_text);
                let b_distance = matcher.fuzzy_match(&b.text, &input_text);
                b_distance.cmp(&a_distance).then(a.text.cmp(&b.text))
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
    let mut items_value = (&*items).clone();
    let items = if items.len() > 15 {
        // set items to the first 15 items
        items_value = items_value
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<DropDownItem>>();
    };

    html! {
    <span class ={css_file_macro ! ("dropdown.css")}>

    <select size="5" id = "editor_dropdown" style ={format ! (" top:{}px; left:{}px", p.y, p.x)}>
    {
    (items_value).clone()
    .into_iter().map( | item | {
        let _item = item.clone();
        html !{<option
            onclick={
                let _item = _item.clone();
                let command = command.clone();
                let position = position.clone();
                let trigger = trigger.clone();
                Callback::from(move |_| {
                    let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
                    let curr_focus: Node = selection.focus_node().unwrap();
                    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();
                    let caret = get_caret_position();
                    range.set_end(&curr_focus, caret);


                    let mut start = caret;
                    let text = curr_focus.node_value().unwrap();

                    while start > 0 {
                        start -= 1;
                        log!(start);
                        log!(text.clone());
                        log!(text.clone().len());
                        log!(text.chars().nth((start) as usize));
                        if text.chars().nth((start) as usize).unwrap() == trigger.chars().nth(0).unwrap() {
                            break;
                        }
                    };

                    range.set_start(&curr_focus, start);
                    range.delete_contents();
                    position.set(None);
                    command.emit((_item.clone(), Some(range)));
                })
            }
            >{item.value}</ option>}
    }).collect::<Html> ()
    }

    </ select>
    </ span>
    }
}
