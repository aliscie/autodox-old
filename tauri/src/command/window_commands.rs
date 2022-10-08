use tauri::State;
use tauri::*;
use tauri;

#[allow(dead_code)]
#[tauri::command]
async fn open_new_window(handle: tauri::AppHandle) {
  let docs_window = tauri::WindowBuilder::new(
    &handle,
    "external", /* the unique window label */
    tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
  ).build().unwrap();
}
