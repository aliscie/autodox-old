use serde_json::json;
use web_sys::MouseEvent;
use yew::Callback;

use shared::invoke;

pub fn on_mouse_move() -> Callback<MouseEvent> {
    Callback::from(|e: MouseEvent| {
        let _ = invoke(
            "mouse_move".to_string(),
            Some(&json!({ "x" : e.x(), "y": e.y()})),
        );
    })
}
