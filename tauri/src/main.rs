#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri::{Runtime, State, Window};

mod command;
//mod entity;
mod utils;
mod tests;
mod error;
mod store;
mod ctx;

#[derive(Debug, Clone, Copy, Default)]
struct MouseLoc {
    x: i32,
    y: i32,
}

struct Storage {
    store: Mutex<HashMap<u32, MouseLoc>>,
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



//pub async fn connect_database(postgres_url: String) -> DatabaseConnection {
    //let db = Database::connect(postgres_url).await;

    //match db {
        //Ok(x) => {
            //return x;
        //}
        //Err(e) => {
            //panic!("Cannot connect to database with url : {}", e);
        //}
    //}
//}

fn main() {
    dotenv().ok();
    //let POSTGRES_URL: &str = &std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    tauri::Builder::default()
        .manage(Storage {
            store: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            maximize_window,
            close_window,
            mouse_move,
            //crate::command::file_command::get_directory,
            //crate::command::file_command::get_directories,
            //crate::command::file_command::create_directory,
            //crate::command::file_command::create_file,
            //crate::command::file_command::delete_file,
            //crate::command::file_command::change_directory,
        ])
        //.setup(|app| {
            //let win = app.get_window("main").unwrap();
            //#[cfg(target_os = "macos")]
                //win.set_transparent_titlebar(true);
            //let handle = app.handle();
            //tauri::async_runtime::block_on(async move {
                //let conn = connect_database(postgres_url).await;
                //handle.manage(conn);
            //});
            //Ok(())
        //})
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
async fn mouse_move(x: i32, y: i32, state: State<'_, Storage>) -> Result<(), ()> {
    let mut w = state.store.lock().await;
    w.insert(0, MouseLoc { x, y });
    println!("{:?}", w.entry(0));
    Ok(())
}
