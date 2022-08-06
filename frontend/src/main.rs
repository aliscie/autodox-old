use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::console::log_1;

mod components;

use components::{TitleBarButton, TitleBar};
use web_sys::{Element, MouseEvent, window, Document};


fn main() {
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn minimize_window(name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}


#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());

    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());

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
    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {

        if x.chars().count() > 1 {
            x.set("".to_string());
        } else {
            x.set("width:250px".to_string());
        }
    });


    html! {
        <div>
        <TitleBar title="AUTODOX">
            <div>
            <TitleBarButton
             onclick={toggle_asidebar}
             button_type="toggle">{"☰"}</TitleBarButton>
            <TitleBarButton button_type="close">{"x"}</TitleBarButton>
            <TitleBarButton button_type="minimize">{"-"}</TitleBarButton>
            <TitleBarButton button_type="zoom">{"+"}</TitleBarButton>
            </div>
        </TitleBar>


        <aside style={format!("{}",(*aside_bar_taggol).clone())}>
          {"content"}
        </aside>

        <h1>{"text"}</h1>

        <article>
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
        <h2 class={"heading"}>{message}</h2>
        </article>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
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

fn min_window(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        match minimize_window(name).await {
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