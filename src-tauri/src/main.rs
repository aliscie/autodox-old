#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::collections::HashMap;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use tauri::async_runtime::Mutex;
use tauri::{Runtime, Window, State};
use tauri::Manager;

#[derive(Debug, Clone, Copy, Default)]
struct MouseLoc{
    x : i32,
    y : i32,
}

struct Storage{
    store : Mutex<HashMap<u32, MouseLoc>>
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

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![hello,minimize_window])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    tauri::Builder::default()
        .manage(Storage { store : Mutex::new(HashMap::new())})
        .invoke_handler(tauri::generate_handler![hello,minimize_window, maximize_window, close_window, mouse_move])
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            // TODO: implement this for linux and windows
            #[cfg(target_os = "macos")]
                win.set_transparent_titlebar(true);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    // This is a very simplistic example but it shows how to return a Result
    // and use it in the front-end.
    if name.contains(' ') {
        Err("Name should not contain spaces".to_string())
    } else {
        Ok(format!("Hello, {}", name))
    }
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

#[tauri::command]
async fn mouse_move(x : i32, y : i32, state : State<'_, Storage>) -> Result<(), ()>{
    let mut w = state.store.lock().await;
    w.insert(0, MouseLoc{ x , y});
    println!("{:?}", w.entry(0));
    Ok(())
}
