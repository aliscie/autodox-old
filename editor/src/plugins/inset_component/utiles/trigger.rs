use crate::plugins;
use crate::plugins::inset_component::types::Position;
use crate::plugins::DropDownItem;
use shared::log;
use std::ops::Deref;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::*;
use yew::prelude::*;
use yew::suspense::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

#[hook]
pub fn use_trigger_popover<F>(
    trigger: String,
    editor_ref: NodeRef,
    popover_ref: NodeRef,
    command: F,
) -> (
    UseStateHandle<Option<Range>>,
    UseStateHandle<Option<Position>>,
    UseStateHandle<String>,
)
where
    F: Fn(Range) + 'static,
{
    let position: UseStateHandle<Option<Position>> = use_state_eq(|| None);
    let range_state: UseStateHandle<Option<Range>> = use_state(|| None);
    let input_text = use_state(|| "".to_string());

    {
        let position = position.clone();
        // if user click anywhere beside the element make it vanish!
        use_click_away(popover_ref.clone(), move |_| {
            if position.is_some() {
                position.set(None);
            }
        });
    }

    {
        let position = position.clone();
        let input_text = input_text.clone();
        let range_state = range_state.clone();
        let mut range = window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .create_range()
            .unwrap();
        let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
        use_event(editor_ref, "keydown", move |e: KeyboardEvent| {
            let curr_focus: Node = selection.focus_node().unwrap();
            if selection.anchor_offset() != 0 {
                let _ = range.set_start(
                    &curr_focus,
                    selection.anchor_offset() - input_text.len() as u32 - trigger.len() as u32,
                );
            } else {
                let _ = range.set_start(&curr_focus, 0);
            }
            let _ = range.set_end(&curr_focus, selection.anchor_offset());
            if position.is_none() && e.key() == trigger {
                if let Some(p) = range.get_client_rects() {
                    let p = p.get(0).unwrap();
                    position.set(Some(Position {
                        x: p.right() + 10 as f64,
                        y: p.y(),
                    }));
                };
            }
            if position.is_some() && (e.key() == "Enter" || e.key() == "Tab") {
                e.prevent_default();
                range.delete_contents();
                position.set(None);
                input_text.set("".to_string());
                command(range.clone());
            }
            if position.is_some() && (e.key() != "Enter" && e.key() != "Tab") {
                match e.key().as_str() {
                    "Backspace" if input_text.as_str() == "" => {
                        // if input text is none and we get a backspace
                        // remove the popover
                        range_state.set(None);
                        position.set(None);
                        input_text.set("".to_string());
                        return;
                    }
                    "Delete" => {
                        range.delete_contents();
                        range_state.set(None);
                        position.set(None);
                        input_text.set("".to_string());
                        return;
                    }
                    "Backspace" => {
                        // removing input
                        let mut text = input_text.deref().to_owned();
                        text.remove(text.len() - 1);
                        input_text.set(text);
                    }
                    // no-op
                    "Ctrl" | "Meta" | "Alt" | "Shift" => {}
                    x => {
                        // pushing new character
                        let mut text = input_text.deref().to_owned();
                        text = format!("{}{}", text, x);
                        input_text.set(text);
                    }
                }
                range_state.set(Some(range.clone()));
                if let Some(p) = range.get_client_rects() {
                    let p = p.get(0).unwrap();
                    position.set(Some(Position {
                        x: p.right() + 10 as f64,
                        y: p.y(),
                    }));
                };
            }
        });
    }
    return (range_state, position, input_text);
}
