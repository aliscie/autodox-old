use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokerFunction, catch)]
    fn invoke_inner(command: String, args: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokerAsyncFunction, catch)]
    async fn invoke_inner_async(command: String, args: String) -> Result<JsValue, JsValue>;
}

pub fn invoke<T: serde::Serialize>(command: String, args: Option<&T>) {
    let res = match args {
        Some(v) => invoke_inner(command, serde_json::to_string(v).unwrap()),
        None => invoke_inner(command, "".into()),
    };
    if let Err(e) = res {
        let window = window().unwrap();
        window
            .alert_with_message(&format!("Error: {:?}", e))
            .unwrap();
    }
}

pub fn invoke_async<T: serde::Serialize>(command: String, args: Option<&T>) {
    let arg: String = match args {
        Some(v) => serde_json::to_string(v).unwrap(),
        None => "".into(),
    };
    spawn_local(async move {
        let res = invoke_inner_async(command, arg).await;
        if let Err(e) = res {
            let window = window().unwrap();
            window
                .alert_with_message(&format!("Error: {:?}", e))
                .unwrap();
        }
    });
}
