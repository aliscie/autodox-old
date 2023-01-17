use std::fmt::Display;

use crate::plugins::editor_toolbar;
use shared::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: Callback<ToolbarAction>,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolbarAction {
    Bold,
    Italic,
    Underscore,
    Comment,
    Droplet,
}

impl Display for ToolbarAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bold => write!(f, "Bold"),
            Self::Italic => write!(f, "Italic"),
            Self::Underscore => write!(f, "Underscore"),
            Self::Comment => write!(f, "Comment"),
            Self::Droplet => write!(f, "Droplet"),
        }
    }
}

#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    use_effect_with_deps(
        move |editor_ref| {
            let _toolbar_action = editor_toolbar();
        },
        (),
    );

    let buttons: [ToolbarAction; 5] = [
        ToolbarAction::Bold,
        ToolbarAction::Italic,
        ToolbarAction::Underscore,
        ToolbarAction::Comment,
        ToolbarAction::Droplet,
    ];
    let action = props.action.clone();
    html! {
    <div contenteditable="false" id="selection-popper" class="buttons_group_class">

        {buttons.into_iter().map(|button| html! {
            <button
                onclick = {
                    let action = action.clone();
                    move |_| action.emit(button)
                }
            ><i class={format!("fa-{}",button)}></i>{button.to_string().chars().nth(0).unwrap()}</button>
        }).collect::<Html>()}
        </div>
    }
}
