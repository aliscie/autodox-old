use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{Element, KeyboardEvent, MouseEvent, Node, window};
use yew::prelude::*;

use shared::*;

#[wasm_bindgen(module = "/src/plugins/inset_component/caret_position.js")]
extern "C" {
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position() -> u32;
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub trigger: String,
}

pub struct Position {
    x: f64,
    y: f64,
}

#[function_component]
pub fn EditorInsert(props: &Props) -> Html {
    let position: UseStateHandle<Option<Position>> = use_state(|| None);
    let input_text: UseStateHandle<String> = use_state(|| "".to_string());
    let _input_text = input_text.clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let text_editor = doc.query_selector(".text_editor").unwrap().unwrap();
    let trigger = props.trigger.clone();
    let _position = position.clone();

    use_effect_with_deps(
        move |editor_ref| {
            insert_components(&text_editor, trigger, _position, _input_text);
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
    let items = vec!["table", "quote", "heading"];
    html! {
        <span class={css_file_macro!("dropdown.css")} >

            <span id="editor_dropdown" style={format!(" top:{}px; left:{}px",p.y,p.x)}>

                {
                items
                .iter()
                .filter(|&element| element.contains(&*input_text))
                .cloned()
                .into_iter().map(|name| {
                    html!{<a key={name}>{ format!("{}",name) }</a>}
                }).collect::<Html>()
            }

            </span>
        </span>
    }
}

pub fn insert_components(editor: &Element, trigger: String, position: UseStateHandle<Option<Position>>, input_text: UseStateHandle<String>) {
    let mut text_value: String = "".to_string();
    let mut track: bool = false;
    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
        let curr_focus: Node = selection.focus_node().unwrap();

        if track && (&e.key() == "Enter" || e.key() == "Tab") {
            e.prevent_default();
            range.set_end(&curr_focus, get_caret_position()).unwrap();
            range.delete_contents();
            position.set(None);
            track = false;
        }

        if e.key() == trigger {
            range.set_start(&curr_focus, get_caret_position());
            range.set_end(&curr_focus, get_caret_position());
            track = true;
        }

        if track {
            range.set_end(&curr_focus, get_caret_position()).unwrap();
            let p = range.get_client_rects().unwrap().get(0);
            input_text.set(format!("{}", range.to_string()).to_string());

            if let Some(p) = p {
                position.set(Some(Position { x: p.right() + 10 as f64, y: p.y() }));
            };
        }
    }) as Box<dyn FnMut(_)>);


    &editor.add_event_listener_with_callback("keyup", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}