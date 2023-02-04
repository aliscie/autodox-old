use crate::backend;
use crate::shared::*;
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
pub fn Share(props: &Props) -> Html {
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
            let _file_mode = _file_mode.clone();
            Box::pin(async move {
                if let Some(file_node) = state.files.vertices.get_mut(&_file_id) {
                    file_node.file_mode = _file_mode;
                    let res = backend::update_file(file_node.clone()).await;
                    if (res.is_ok()) {
                        log!("Do something...");
                    }
                }
            })
        });

    html! {
        <div class={css_file_macro!("share.css")}>
            {"Share"}
        </div>

    }
}
