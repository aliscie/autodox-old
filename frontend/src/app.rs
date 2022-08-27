use crate::utils::alert;
// use backend::get_users::get_data;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;


#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use crate::components::{TitleBar, TitleBarButton};

use crate::components::TreeList;
use web_sys::{window, Document, Element, MouseEvent};


#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());

    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());
    //spawn_local(async{
    //log_1(&format!("{:?}" ,get_data().await).into());
    //});

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

    html! {
        <div>
        { super::utils::get_titlebar(article_position.clone(), x) }
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
    if name.contains(' ') {
        let window = window().unwrap();
        window
            .alert_with_message("Error: Name should not contain sapces!")
            .unwrap();
    } else {
        welcome.set(name);
    }
}

