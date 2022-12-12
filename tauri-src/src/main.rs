#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use dotenv::dotenv;

use context::Context;
use serde_json::Value;
use store::Store;
use tauri::{Manager, Runtime, Window};

mod command;
mod prelude;
//mod entity;
mod context;
mod error;
mod store;
mod tests;
mod utils;

use crate::prelude::*;
use crate::utils::map;

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

fn main() -> Result<()> {
    dotenv().ok();
    tauri::Builder::default()
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
            crate::command::element::update_element,
            crate::command::element::delete_element,
        ])
        .setup(|app| {
            // if not found use memory
            let mut database_url = String::from("memory");
            match app.get_cli_matches() {
                Ok(mut matches) => {
                    let entry = matches
                        .args
                        .remove(&"database_url".to_string())
                        .unwrap_or_default();
                    match entry.value {
                        Value::String(v) => {
                            database_url = v;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
            let store = tauri::async_runtime::block_on(async { Store::new().await }).unwrap();
            app.manage(Context::new(store));
            let win = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            win.set_transparent_titlebar(true);
            #[cfg(debug_assertions)]
            win.open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri-src application");
    Ok(())
}

#[tauri::command]
fn close_window(window: Window) -> tauri::Result<()> {
    return Ok(window.close()?);
}

#[tauri::command]
fn minimize_window(window: Window) -> tauri::Result<()> {
    return Ok(window.minimize()?);
}

#[tauri::command]
fn maximize_window(window: Window) -> tauri::Result<()> {
    if window.is_fullscreen()? {
        return Ok(window.set_fullscreen(false)?);
    }
    Ok(window.set_fullscreen(true)?)
}

// #[tauri-src::command]
// async fn mouse_move(x: i32, y: i32, ctx: State<'_, Context>) -> Result<(), ()> {
//     let mut w = ctx.store.mouse_loc.lock().await;
//     w.insert(0, MouseLoc { x, y });
//     println!("{:?}", w.entry(0));
//     Ok(())
// }
