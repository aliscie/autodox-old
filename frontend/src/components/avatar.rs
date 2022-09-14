use yew::{function_component, html, Html};

#[function_component(Avatar)]
pub fn avatar() -> Html {
    html! {
    <>
    <img src="https://avatars.githubusercontent.com/u/58806996?v=4" alt="Avatar" class="avatar"/>
    // <span class="active-icon"></span>
    </>
 }
}
