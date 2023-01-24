use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::functional::use_store;
use crate::shared::*;
use crate::utils::{DeviceInfo, Image};
use crate::components::{ToolTip, Loading};

#[derive(Properties, PartialEq)]
pub struct Props {}

use crate::components::{Avatar};

#[function_component]
pub fn ProfileInfo(props: &Props) -> Html {
    let show: UseStateHandle<bool> = use_state(|| false);

    let (device, _) = use_store::<DeviceInfo>();
    let profile = device.profile.clone();
    let profile_object = serde_json::json!(device.profile);
    if device.profile.username.is_none() {
        return html! {<Loading/>};
    }
    let profile_object = profile_object.as_object().unwrap().iter().filter(|(k, v)| { !["image", "username"].contains(&&***k) });
    let onsubmit: Callback<SubmitEvent> = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
        log!("xxx");
    });

    let _show = show.clone();
    let onkeydown: Callback<KeyboardEvent> = Callback::from(move |_e: KeyboardEvent| {
        if _e.key() == " " {
            _show.set(true);
            _e.prevent_default();
        } else {
            if *_show { _show.set(false) };
        }
    });
    let _show = show.clone();
    html! {<form
        {onsubmit}
        class={css_file_macro!("profile_info.css")}
        >
        <Avatar size={Some(150)} src={Image::get_opt_link(profile.image.clone())}/>
        <ToolTip show={Some(*_show)} context={html!{<span>{"Spaces are not allowed"}</span>}}>
            <h2 {onkeydown} contenteditable="true" name="username" >{profile.username.unwrap()}</h2>
        </ToolTip>

            <table>
              {
                profile_object.map(|(k,v)| {
                html!{<tr>
                  <td>{k}</td>
                  <td name={format!("{}",k)} contenteditable="true" >{v}</td>
                </tr>}
                }).collect::<Html>()

              }
            </table>
        <button type="submit" value="Submit" style="width:100%" class="btn">{"Save"}</button>
        </form>}
}
