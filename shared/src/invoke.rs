use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokerFunction, catch)]
    fn invoke_inner(command: String, args: String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = invokerAsyncFunction, catch)]
    async fn invoke_inner_async(command: String, args: String) -> Result<JsValue, JsValue>;
}

pub fn invoke<T: serde::Serialize>(command: String, args: Option<&T>) -> Result<JsValue, JsValue> {
    let res = match args {
        Some(v) => invoke_inner(command, serde_json::to_string(v).unwrap()),
        None => invoke_inner(command, "".into()),
    };
    return res;
}

pub async fn invoke_async<T: serde::Serialize>(
    command: String,
    args: Option<&T>,
) -> Result<JsValue, JsValue> {
    let arg: String = match args {
        Some(v) => serde_json::to_string(v).unwrap(),
        None => "".into(),
    };
    let res = invoke_inner_async(command, arg).await;
    return res;
}
