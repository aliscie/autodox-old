use utils::{alert, invoke_async};
// use backend::get_users::get_data;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod backend;
mod components;
mod extensions;
mod test;
mod utils;

use crate::utils::invoke;
use components::{TitleBar, TitleBarButton, TreeList};
use web_sys::{window, Document, Element, MouseEvent};

// mod backend;
// use backend::{get_data};
fn main() {
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    // #[wasm_bindgen(js_name = invokeHello, catch)]
    // pub async fn minimize_window(name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());

    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());
    //spawn_local(async{
    //log_1(&format!("{:?}" ,get_data().await).into());
    //});

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let message = (*welcome).clone();
    let x = aside_bar_taggol.clone();
    let article_position_value = article_position.clone();

    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            article_position_value.set("".to_string());
        } else {
            x.set("width:250px".to_string());
            article_position_value.set("margin-left:270px".to_string());
        }
    });

    let toggle_maximize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let _ = invoke::<String>("maximize_window".into(), None).map_err(|e| alert(&e));
    });
    let toggle_minimize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let _ = invoke::<String>("minimize_window".into(), None).map_err(|e| alert(&e));
    });

    let close_window: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let _ = invoke::<String>("close_window".into(), None).map_err(|e| alert(&e));
    });

    html! {
        <div>
        <TitleBar title="current_path/current_file">
            <div
            // style="margin-left:60px"
            >
            <TitleBarButton onclick = {close_window} button_type="close">{""}</TitleBarButton>
            <TitleBarButton onclick = {toggle_minimize} button_type="minimize">{""}</TitleBarButton>
            <TitleBarButton
            onclick={toggle_maximize}
            button_type="zoom">{""}</TitleBarButton>
            <TitleBarButton onclick={toggle_asidebar} button_type="toggle">{""}</TitleBarButton>
            // <TitleBarButton button_type="next_back">{"←"}</TitleBarButton>
            // <TitleBarButton button_type="next_back">{"→"}</TitleBarButton>
            // <TitleBarButton button_type="share">{"⤿"}</TitleBarButton>
            // <TitleBarButton button_type="star">{"★"}</TitleBarButton>
            // <TitleBarButton button_type="toggle">{"👨‍💼"}</TitleBarButton>
            // <TitleBarButton button_type="toggle">{"💬"}</TitleBarButton>
            // <TitleBarButton button_type="toggle">{"..."}</TitleBarButton>
            </div>
        </TitleBar>


        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL"><TreeList
        // files={files}
        /></ul>

        </aside>

        <article contenteditable="true" style={format!("{}",(*article_position).clone())}>
        <h2 class={"heading"}>{message}</h2>
        {r"
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Maxime mollitia,
            molestiae quas vel sint commodi repudiandae consequuntur voluptatum laborum
            numquam blanditiis harum quisquam eius sed odit fugiat iusto fuga praesentium
            optio, eaque rerum! Provident similique accusantium nemo autem. Veritatis
            obcaecati tenetur iure eius earum ut molestias architecto voluptate aliquam
            nihil, eveniet aliquid culpa officia aut! Impedit sit sunt quaerat, odit,
            tenetur error, harum nesciunt ipsum debitis quas aliquid. Reprehenderit,
            quia. Quo neque error repudiandae fuga? Ipsa laudantium molestias eos
            sapiente officiis modi at sunt excepturi expedita sint? Sed quibusdam
            recusandae alias error harum maxime adipisci amet laborum. Perspiciatis
            minima nesciunt dolorem! Officiis iure rerum voluptates a cumque velit
            quibusdam sed amet tempora. Sit laborum ab, eius fugit doloribus tenetur
            fugiat, temporibus enim commodi iusto libero magni deleniti quod quam
            consequuntur! Commodi minima excepturi repudiandae velit hic maxime
            doloremque. Quaerat provident commodi consectetur veniam similique ad
            earum omnis ipsum saepe, voluptas, hic voluptates pariatur est explicabo
            fugiat, dolorum eligendi quam cupiditate excepturi mollitia maiores labore
            suscipit quas? Nulla, placeat. Voluptatem quaerat non architecto ab laudantium
            modi minima sunt esse temporibus sint culpa, recusandae aliquam numquam
            totam ratione voluptas quod exercitationem fuga. Possimus quis earum veniam
            quasi aliquam eligendi, placeat qui corporis!
        "}
        </article>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // let x = main.await;
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match invoke_async("hello".to_string(), Some(&json!({ "name": name }))).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}
