use web_sys::MouseEvent;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TableContextMenuProps {
    pub event: MouseEvent,
    // table_id: Id,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TableContextMenu(props: &TableContextMenuProps) -> Html {
    html! {
    <>
        {props.children.clone()}
    </>
    }
}
