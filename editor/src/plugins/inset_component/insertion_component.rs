use shared::*;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, Element, KeyboardEvent, MouseEvent, Node, Range};
use yew::prelude::*;
use yew::suspense::{SuspensionResult, UseFutureHandle};
use yew_hooks::prelude::*;

use crate::plugins::inset_component::types::{CommandItems, DropDownItem, Position};
use crate::plugins::inset_component::utiles;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use super::use_trigger_popover;

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
    pub editor_ref: NodeRef,
}

#[function_component]
pub fn EditorInsert(props: &Props) -> Html {
    let items = use_state_eq(|| props.items.clone());
    let command = props.command.clone();
    let trigger = props.trigger.clone();
    let editor_ref = props.editor_ref.clone();
    let node = NodeRef::default();
    let (range_state, position, input_text) =
        use_trigger_popover(trigger, editor_ref, node.clone(), {
            let items = items.clone();
            let command = command.clone();
            move |r: Range| {
                command.emit((items[0].clone(), Some(r)));
            }
        });
    {
        let items = items.clone();
        // fuzzy matching each time input_text changes
        use_effect_with_deps(
            move |input_text| {
                let matcher = SkimMatcherV2::default();
                let mut sorted_items = items.deref().to_owned();
                sorted_items.sort_unstable_by(|a, b| {
                    let a_distance = matcher.fuzzy_match(&a.text, &input_text);
                    let b_distance = matcher.fuzzy_match(&b.text, &input_text);
                    b_distance.cmp(&a_distance).then(a.text.cmp(&b.text))
                });
                items.set(sorted_items.clone());
            },
            input_text.clone(),
        );
    }
    if position.is_none() {
        return html! {};
    }
    let p = position.as_ref().unwrap();

    html! {
    <span class ={css_file_macro ! ("dropdown.css")} ref = {node.clone()}>

    <select size="5" id = "editor_dropdown" style ={format ! (" top:{}px; left:{}px", p.y, p.x)}>
    {
        items
            .deref()
            .into_iter()
            .take(25)
            .map(|item| {
                html !{
                    <option
                        onclick = {
                        let item = item.clone();
                        let command = command.clone();
                        let position = position.clone();
                        let range_state = range_state.clone();
                        let input_text = input_text.clone();
                        Callback::from(move |_ | {
                            position.set(None);
                            input_text.set("".to_string());
                            if let Some(range) = range_state.deref() {
                                range.delete_contents();
                            }
                            command.emit((item.clone(), range_state.deref().clone()));
                        })
                    }
                    >{&item.value}
                    </option>
                }
            }).collect::<Html> ()
    }

    </ select>
    </ span>
    }
}
