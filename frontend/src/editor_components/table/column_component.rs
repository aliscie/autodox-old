use web_sys::{HtmlElement, MouseEvent};
use yew::{Callback, Children, html, Html};
use yew::prelude::*;
use editor::GlobalEditorState;
use shared::schema::EditorElement;
use super::table_context_menu::*;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
    pub element: EditorElement,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Column(props: &Props) -> Html {
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");
    let on_column_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let curr: HtmlElement = mouse_event.target_unchecked_into();

            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![
                    html!{<><a
                        onclick={
                            Callback::from(move |e: MouseEvent| {
                                curr.set_attribute("formula", "1+1").unwrap_throw();
                            })
                        }
                        >{"add formula"}</a>
                        <a>{"add filter"}</a>
                        <a>{"somthing"}</a>
                    </>},
                    ]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state
                .render_context_menu
                .emit((mouse_event, element))
        })
    };
    let id = props.element.id.clone();
    let content = props.element.content.clone();
    html! {<th
                oncontextmenu={on_column_contextmenu.clone()}
                    id = {id.to_string()}>
                    {content}
                    {props.children.clone()}
                </th>}
}