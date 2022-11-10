#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use dotenv::dotenv;

use context::Context;
use store::Store;
use tauri::{Runtime, Window};

mod command;
mod prelude;
//mod entity;
mod context;
mod error;
mod model;
mod store;
mod tests;
mod utils;

#[derive(Debug, Clone, Copy, Default)]
pub struct MouseLoc {
    x: i32,
    y: i32,
}

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        use cocoa::appkit::NSWindowTitleVisibility;

        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;

            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                transparent,
            );
            id.setStyleMask_(style_mask);

            id.setTitleVisibility_(if transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });
            id.setTitlebarAppearsTransparent_(if transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let store = Store::new()
        .await
        .expect("Cannot create connection to database!");
    tauri::Builder::default()
        .manage(Context::new(store))
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            maximize_window,
            close_window,
            //mouse_move,
            crate::command::file_command::get_directory,
            crate::command::file_command::get_directories,
            crate::command::file_command::create_directory,
            crate::command::file_command::create_file,
            crate::command::file_command::delete_file,
            crate::command::file_command::change_directory,
            crate::command::element::get_element_tree,
            crate::command::element::create_element_tree,
            crate::command::element::create_element,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                let win = app.get_window("main").unwrap();
                win.set_transparent_titlebar(true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri-src application");
}

#[tauri::command]
fn close_window(window: Window) -> Result<(), tauri::Error> {
    return window.close();
}

#[tauri::command]
fn minimize_window(window: Window) -> Result<(), tauri::Error> {
    return window.minimize();
}

#[tauri::command]
fn maximize_window(window: Window) -> Result<(), tauri::Error> {
    if window.is_fullscreen().unwrap() {
        return window.set_fullscreen(false);
    }
    window.set_fullscreen(true)
}

// #[tauri-src::command]
// async fn mouse_move(x: i32, y: i32, ctx: State<'_, Context>) -> Result<(), ()> {
//     let mut w = ctx.store.mouse_loc.lock().await;
//     w.insert(0, MouseLoc { x, y });
//     println!("{:?}", w.entry(0));
//     Ok(())
// }
