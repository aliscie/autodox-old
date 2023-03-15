use yew::{function_component, Html, html, Properties};
use web_sys::MouseEvent;

#[derive(Properties, PartialEq)]
pub struct TableContextMenuProps {
    pub event: MouseEvent,
    // table_id: Id,
    pub items: Vec<Html>,
}

#[function_component]
pub fn ContextMenu(props: &TableContextMenuProps) -> Html {
    let x = props.event.page_x();
    let y = props.event.page_y();
    let style = format!("position: absolute; display : block; top:{}px; left:{}px;", &y, &x);

    html! {
        <div {style} class={"dropdown-content"}>
        {props.items.clone()}
        </div>
    }
}
