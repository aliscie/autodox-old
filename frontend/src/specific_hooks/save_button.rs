use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct SaveButtonState {
    pub is_saved: bool,
    // element: EditorElementUpdate,
}
