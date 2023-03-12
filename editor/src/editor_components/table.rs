use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use yew::{function_component, html};
use shared::id::Id;
use shared::schema::EditorElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Option<Vec<Id>>,
}

#[function_component]
pub fn Table(props: &Props) -> Html {
    let filter_options: Vec<Html> = vec![
        html! { <option>{ "my filter" }</option>} ,
        html! { <option>{ "other filer" }</option> },
        html! { <option>{ "todos filer" }</option> },
        html! { <option>{ "Add a filter +" }</option> },
    ];
    let view_options: Vec<Html> = vec![
        html! { <option>{ "grid" }</option> },
        html! { <option>{ "other" }</option> },
        html! { <option>{ "blab_blab" }</option> },
        html! { <option>{ "Add a view +" }</option> },
    ];

    html! {
        <div>
            <div style="background: gray">
                <span>{"hello"}</span> 
                <select>
                    {filter_options}
                </select> 
                <select> 
                    {view_options}
                </select>
            </div>

            <table>
                // {props.children.into_iter().map(|row| {
                //     html! {
                //         <tr>
                //             {map(|cell| {<td>{cell}</td>}}
                //
                //         </tr>
                //     }
                // }).collect::<Html>()}
            </table>
        </div>
    }
}
