use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct SaveButtonState {
    pub style: String,
    // element: EditorElementUpdate,
}
