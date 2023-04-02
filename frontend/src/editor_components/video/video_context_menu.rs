use web_sys::MouseEvent;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TableContextMenuProps {
    pub event: MouseEvent,
    // table_id: Id,
    pub items: Vec<Html>,
}

#[function_component]
pub fn ContextMenu(props: &TableContextMenuProps) -> Html {
    html! {
        <>
        {props.items.clone()}
        </>
    }
}
