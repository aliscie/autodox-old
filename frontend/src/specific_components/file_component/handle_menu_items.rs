use yew::prelude::*;
use yew::suspense::*;
use yew_router::prelude::*;
use yewdux::functional::use_store;
use yewdux::prelude::*;
use web_sys::{Element, HtmlInputElement, KeyboardEvent, MouseEvent};
use shared::schema::FileMode;
use crate::{backend, components::PopOverMenu, router::Route};

use shared::{
    id::Id,
    log,
    schema::{FileDirectory, FileNode, FileNodeDelete},
};
use crate::specific_components::file_component::FileComponentProps;
use yew_router::prelude::{use_navigator, use_route};


#[hook]
pub fn use_file_items(props: &FileComponentProps) -> Vec<Html> {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or_default();
    let (fd_rc, fd_dispatch) = use_store::<FileDirectory>();
    let id = props.id.clone();
    let _id = id.clone();

    let onkeydown: Callback<KeyboardEvent> =
        fd_dispatch.reduce_mut_future_callback_with(move |state, _e: KeyboardEvent| {
            Box::pin(async move {
                let input: HtmlInputElement = _e.target_unchecked_into();
                let value: String = input.inner_text();

                if _e.key() == "Enter" {
                    _e.prevent_default();
                };

                if _e.key() != "Enter" {
                    input.class_list().remove_1("tool").unwrap();
                };

                let clone_id = _id.clone();

                if _e.key() == "Enter" && value.is_empty() {
                    input.class_list().add_1("tool").unwrap();
                } else if _e.key() == "Enter" {
                    let res = backend::rename_file(_id.clone(), value.clone()).await;
                    if (res.is_ok()) {
                        state.files.vertices.get_mut(&_id).unwrap().name = value;
                    }
                }
            })
        });

    let on_permission = {
        let _navigator = navigator.clone();
        let _id = props.id.clone();
        // move |_| _navigator.push(&PagesRoute::Permission { id: _id })
    };

    let _id = props.id.clone();
    let _fd_rc = fd_rc.clone();
    let _navigator = navigator.clone();
    let on_share = Callback::from(move |_e: MouseEvent| {
        let file_mode = _fd_rc
            .clone()
            .files
            .vertices
            .get(&_id)
            .unwrap_or(&FileNode::default())
            .file_mode
            .clone();
        if (file_mode == FileMode::Restricted) {
            // _navigator.push(&PagesRoute::Share { id: _id });
        } else {
            log!("Can't share since user isn't restricted");
        }
    });
    let handle_public = {
        // let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {})
    };

    let handle_private = {
        // let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {})
    };

    let ondelete = {
        let id = id.clone();
        let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
        let mut parent_id = Id::default();
        for (parent, child_id) in &_dispatch_file_directory.get().files.adjacency {
            if child_id.contains(&id) {
                parent_id = *parent;
            }
        }
        let delete_file_node = FileNodeDelete {
            id,
            parent_id,
            tree_id: _dispatch_file_directory.get().id,
        };
        let file_id = id.clone();
        let _navigator = navigator.clone();
        _dispatch_file_directory.reduce_mut_future_callback(move |state| {
            match route {
                // the current file is in use navigate to home!
                Route::File { id, auther: _ } => {
                    if file_id == id {
                        _navigator.push(&Route::Home);
                    }
                }
                _ => {}
            }
            Box::pin(async move {
                let result = crate::backend::delete_file(delete_file_node).await;

                log!(result);
                state.files.remove(&file_id);
            })
        })
    };

    let _id = id.clone();
    let onkeydown: Callback<KeyboardEvent> =
        fd_dispatch.reduce_mut_future_callback_with(move |state, _e: KeyboardEvent| {
            Box::pin(async move {
                let input: HtmlInputElement = _e.target_unchecked_into();
                let value: String = input.inner_text();

                if _e.key() == "Enter" {
                    _e.prevent_default();
                };

                if _e.key() != "Enter" {
                    input.class_list().remove_1("tool").unwrap();
                };

                let clone_id = _id.clone();

                if _e.key() == "Enter" && value.is_empty() {
                    input.class_list().add_1("tool").unwrap();
                } else if _e.key() == "Enter" {
                    let res = backend::rename_file(_id.clone(), value.clone()).await;
                    if (res.is_ok()) {
                        state.files.vertices.get_mut(&_id).unwrap().name = value;
                    }
                }
            })
        });
    return vec![
        html! {<a><span
                {onkeydown}
                contenteditable="true"
                data-tip="File must have at least 1 character."
                autofocus=true placeholder="rename..">{props.name.clone()}</span></a>},
        // html! {<a onclick={on_share}><i class="fa-solid fa-upload"/>{"Share"}</a>},
        // html! {<a onclick={on_permission}><i class="fa-solid fa-eye"/>{"Permissions"}</a>},
        html! {<a onclick = {ondelete}><i class="fa-solid fa-trash"/>{"Delete"}</a>},
        html! {<a><i class="fa-brands fa-medium"></i>{"Category"}</a>},
        html! {<a ><i onclick = {handle_public}  class="fa-solid fa-eye"/>{"Public"}</a>},
        html! {<a ><i onclick = {handle_private} class="fa-solid fa-eye-slash"/>{"Private"}</a>},
    ];
}