// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window_custom;
use tauri::Manager;
use window_custom::macos::WindowExtMacos;

// NOTE: league sets it's window to 1000 so we go one higher
pub const HIGHER_LEVEL_THAN_LEAGUE: i32 = 1001;

fn main() {

  let mut app = tauri::Builder::default();

  app = app
    // FIXME: fi we disable this plugin it will work
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_nspanel::init());

  app
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      // FIXME: this is crashing the app for some reason?
      window.set_float_panel(HIGHER_LEVEL_THAN_LEAGUE);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
