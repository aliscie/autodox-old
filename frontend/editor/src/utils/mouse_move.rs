use serde_json::json;
use web_sys::MouseEvent;
use yew::Callback;

use crate::utils::invoke;

pub fn on_mouse_move() -> Callback<MouseEvent> {
    Callback::from(|e: MouseEvent| {
        let _ = invoke(
            "mouse_move".to_string(),
            Some(&json!({ "x" : e.x(), "y": e.y()})),
        );
    })
}
