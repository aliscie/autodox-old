use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Element, Node};
use yew::prelude::*;
use yew_hooks::prelude::*;

use super::Position;

#[derive(Properties, PartialEq)]
pub struct ContextMenuProps {
    pub position: Option<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

pub enum ContextMenuMessage {
    Click(MouseEvent),
}

pub struct ContextMenu {
    pub position: Option<MouseEvent>,
    pub menu_ref: NodeRef,
    pub listener: Option<EventListener>,
}

impl Component for ContextMenu {
    type Message = ContextMenuMessage;
    type Properties = ContextMenuProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            position: ctx.props().position.clone(),
            menu_ref: NodeRef::default(),
            listener: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.position != ctx.props().position {
            self.position = ctx.props().position.clone();
            return true;
        }
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ContextMenuMessage::Click(event) => {
                let element = self.menu_ref.cast::<Node>();
                let clicked_element = ctx
                    .props()
                    .position
                    .clone()
                    .map(|p| p.target_unchecked_into::<Element>());
                let target = &event.target_unchecked_into::<Element>();
                let target_node = &event.target_unchecked_into::<Node>();
                let mut is_click_on: bool = element.clone().unwrap().contains(Some(target_node));
                if let Some(element) = &clicked_element.clone() {
                    if element != target && !is_click_on {
                        self.position = None;
                        return true;
                    }
                }
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let document = window().unwrap().document().unwrap();
        let callback = ctx
            .link()
            .callback(|e: MouseEvent| ContextMenuMessage::Click(e));
        self.listener = Some(EventListener::new(
            &window().unwrap(),
            "click",
            move |event| {
                callback.emit(JsValue::from(event).into());
            },
        ));
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = match &self.position {
            Some(p) => format!("display: block; top : {}px; left:{}px;", p.y(), p.x()),
            None => "display: None;".to_string(),
        };
        html! {
            <div ref = {self.menu_ref.clone()} style = {style} class="dropdown-content">
                {ctx.props().children.clone()}
            </div>
        }
    }
}
