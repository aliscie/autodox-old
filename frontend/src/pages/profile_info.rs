use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::functional::use_store;
use crate::shared::*;
use crate::utils::{DeviceInfo, Image};
use wasm_bindgen_futures::spawn_local;
use crate::backend;
use shared::schema::UserQuery;

#[derive(Properties, PartialEq)]
pub struct Props {}

use crate::components::{Avatar};

#[function_component]
pub fn ProfileInfo(props: &Props) -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    let profile = device.profile.clone();
    let profile_object = serde_json::json!(device.profile);
    if device.profile.address.contains("000") {
        return html! {<div class="profile loader"/>};
    }
    let _profile = profile.clone();
    let profile_object = profile_object.as_object().unwrap().iter().filter(|(k, v)| { !["image", "username"].contains(&&***k) });


    let onsubmit: Callback<MouseEvent> = {
        let mut profile_data = profile.clone();
        Callback::from(move |e: MouseEvent| {
            let curr: Element = e.target_unchecked_into();
            e.prevent_default();
            let mut profile_data = profile_data.clone();
            spawn_local(async move {
                curr.class_list().add_1("loader");
                let profile_json = serde_json::json!(profile_data);
                let res = backend::call_ic("update_profile".to_string(), profile_json.to_string()).await;
                curr.class_list().remove_1("loader");
            });
        })
    };


    let onkeydown: Callback<KeyboardEvent> = {
        let dispatch = dispatch.clone();
        Callback::from(move |_e: KeyboardEvent| {
            let curr: HtmlInputElement = _e.target_unchecked_into();
            if _e.key() == "Enter" {
                _e.prevent_default();
            };
            if _e.key() == " " {
                _e.prevent_default();
                curr.class_list().add_1("tool").unwrap();
            } else if curr.class_list().contains("tool") {
                curr.class_list().remove_1("tool").unwrap();
            }
            let _ = dispatch.reduce_mut(|state| {
                state.profile.username = Some(curr.inner_html().replace(" ", "_"));
            });
        })
    };
    // let onkeydown: Callback<KeyboardEvent> =

    html! {<form
        class={css_file_macro!("profile_info.css")}
        >
        <Avatar size={Some(150)} src={Image::get_opt_link(profile.image.clone())}/>
        <h2 data-tip="Spaces are not allowed." tabindex="1"
        {onkeydown} contenteditable="true" name="username" >{profile.username.unwrap_or("set_your_username".to_string())}</h2>
            <table>
              {
                profile_object.map(|(k,v)| {
                html!{<tr>
                  <td>{k}</td>
                  <td
                  onkeydown={
                        let dispatch = dispatch.clone();
                        let k = k.clone();
                        Callback::from(move |_e: KeyboardEvent| {
                                let curr: HtmlInputElement = _e.target_unchecked_into();
                              let _ = dispatch.reduce_mut(|state| {
                                state.profile.update(&k, curr.inner_html());
                            });
                        })
                  }
                  name={format!("{}",k)} contenteditable="true" >{v}</td>
                </tr>}
                }).collect::<Html>()

              }
            </table>
        <button onclick={onsubmit} type="submit" value="Submit" style="width:100%" class="btn">{"Save"}</button>
        </form>}
}
