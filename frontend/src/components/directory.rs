use crate::router::Route;
use shared::id::Id;
use shared::schema::FileDirectory;
use web_sys::{DragEvent, MouseEvent};
use yew::html;
use yew::prelude::*;
use yew_router::prelude::use_route;
use yewdux::prelude::use_store;

#[function_component(CurDirectory)]
pub fn cur_directory() -> Html {
    let _display = use_state(|| "display: none;".to_string());
    let (file_tree, _) = use_store::<FileDirectory>();
    let route = use_route::<Route>().unwrap_or_default();
    let mut path: Vec<Id> = match route {
        Route::File { id } => file_tree
            .files
            .find_path(file_tree.files.root.as_ref().unwrap(), &id.into()),
        _ => Vec::new(),
    };
    // remove root
    if path.len() > 0 {
        path.remove(0);
    }

    let on_mouse_down = {
        let display = _display.clone();
        move |e: MouseEvent| {
            // display.set("display: block".to_string());
        }
    };

    let on_drag_start = {
        move |e: DragEvent| {
            // opacity:0.5
        }
    };

    let on_drag_end = {
        move |e: DragEvent| {
            // opacity:1
        }
    };

    let on_drag_enter = {
        move |e: DragEvent| {
            // background:lightblue
        }
    };

    let on_drag_leave = {
        move |e: DragEvent| {
            // background:none
        }
    };

    html! {
        <span>
            <span class="hovering file_component">
                {"parent file test 1"}
            </span>
            {
                path.into_iter().map(|f| {
                    html!{
                        <>
                            {"/"}
                            <span class="hovering file_component">
                                {&file_tree.files.vertices.get(&f).unwrap().name}
                            </span>
                        </>
                    }
                }).collect::<Html>()
            }
            <span class="btn" style="width: 35px"><i class="fa-solid fa-share"></i></span>
        </span>
    }
}
