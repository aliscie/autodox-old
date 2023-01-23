use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::functional::use_store;
use crate::shared::*;
use crate::utils::DeviceInfo;
use crate::components::Loading;
#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(ProfileInfo)]
pub fn profile_info(props: &Props) -> Html {
    let (device, _) = use_store::<DeviceInfo>();
    let profile = serde_json::json!(device.profile);
    if device.profile.username.is_none() {
        return html! {<Loading/>};
    }
    html! {<span
        class={css_file_macro!("profile_info.css")}
        >
            <table>
              {
                profile.as_object().unwrap().iter().map(|(k,v)| {
                html!{<tr>
                  <td>{k}</td>
                  <td contenteditable="true" >{v}</td>
                </tr>}
                }).collect::<Html>()

              }
            </table>
        <span style="width:100%" class="btn">{"Save"}</span>
        </span>}
}
