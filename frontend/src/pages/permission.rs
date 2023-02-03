use crate::backend;
use crate::shared::*;
use js_sys::Math::log10;
use shared::id::Id;
use shared::schema::FileDirectory;
use shared::schema::FileMode;
use shared::schema::FileNode;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_id: Id,
}

#[function_component]
pub fn Permission(props: &Props) -> Html {
    let (fd_rc, fd_dispatch) = use_store::<FileDirectory>();
    let file_mode_state = use_state(|| FileMode::Public);
    let file_mode = (*file_mode_state).clone();

    let _file_mode_state = file_mode_state.clone();
    let _fd_rc = fd_rc.clone();
    let _file_id = props.file_id.clone();
    use_future(move || async move {
        _file_mode_state.set(
            _fd_rc
                .clone()
                .files
                .vertices
                .get(&_file_id)
                .unwrap_or(&FileNode::default())
                .file_mode
                .clone(),
        );
    });

    let _file_mode_state = file_mode_state.clone();
    let on_radio_permission = Callback::from(move |_e: MouseEvent| {
        let input: HtmlInputElement = _e.target_unchecked_into();
        let value = input.value();
        if (value == "Private") {
            log!("Private");
            _file_mode_state.set(FileMode::Private);
        }
        if (value == "Restricted") {
            log!("Restricted");
            _file_mode_state.set(FileMode::Restricted);
        }
        if (value == "Public") {
            log!("Public");
            _file_mode_state.set(FileMode::Public);
        }
    });

    let _file_id = props.file_id.clone();
    let _file_mode = file_mode.clone();
    let on_btn_save: Callback<MouseEvent> =
        fd_dispatch.reduce_mut_future_callback_with(move |state, _e: MouseEvent| {
            Box::pin(async move {
                if let Some(file_node) = state.files.vertices.get_mut(&_file_id) {
                    file_node.file_mode = FileMode::Public; // Todo
                    let res = backend::update_file(file_node.clone()).await;
                    if (res.is_ok()) {
                        log!("Do something...");
                    }
                }
            })
        });

    html! {
        <div class={css_file_macro!("permission.css")}>
            <div class="container">
                <div class="radio-group">
                    <div>
                        <input type="radio" id="private" name="permission" value="Private" checked={file_mode==FileMode::Private} onclick={on_radio_permission.clone()}/>
                        <label class="cursor-pointer" for="private">{"Private"}</label>
                    </div>
                    <div>
                        <input type="radio" id="restricted" name="permission" value="Restricted" checked={file_mode==FileMode::Restricted} onclick={on_radio_permission.clone()}/>
                        <label class="cursor-pointer" for="restricted">{"Restricted"}</label>
                    </div>
                    <div>
                        <input type="radio" id="public" name="permission" value="Public" checked={file_mode==FileMode::Public} onclick={on_radio_permission.clone()}/>
                        <label class="cursor-pointer" for="public">{"Public"}</label>
                    </div>
                </div>
                <button class="btn" onclick={on_btn_save}>{"Save"}</button>
            </div>
        </div>

    }
}
