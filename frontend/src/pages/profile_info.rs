use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::functional::use_store;
use crate::shared::*;
use crate::utils::{DeviceInfo, Image};
use crate::components::Loading;

#[derive(Properties, PartialEq)]
pub struct Props {}

use crate::components::{Avatar, PopOverMenu};

#[function_component(ProfileInfo)]
pub fn profile_info(props: &Props) -> Html {
    let (device, _) = use_store::<DeviceInfo>();
    let profile = device.profile.clone();
    let profile_object = serde_json::json!(device.profile);
    if device.profile.username.is_none() {
        return html! {<Loading/>};
    }
    let profile_object = profile_object.as_object().unwrap().iter().filter(|(k, v)| { !["image", "username"].contains(&&***k) });
    let onsubmit: Callback<SubmitEvent> = Callback::from(|e:SubmitEvent| {
        e.prevent_default();
        log!("xxx");
    });
    html! {<form
        {onsubmit}
        class={css_file_macro!("profile_info.css")}
        >
        <Avatar size={Some(150)} src={Image::get_opt_link(profile.image.clone())}/>
        <h2 contenteditable="true" name="username" >{profile.username.unwrap()}</h2>
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
