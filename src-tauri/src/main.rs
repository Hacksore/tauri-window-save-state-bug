// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window_custom;
use tauri::Manager;
use tauri_plugin_window_state::StateFlags;
use window_custom::macos::WindowExtMacos;

// NOTE: league sets it's window to 1000 so we go one higher
pub const HIGHER_LEVEL_THAN_LEAGUE: i32 = 1001;

fn main() {
  let mut flags = StateFlags::all();
  flags.remove(StateFlags::VISIBLE);

  let window_state_plugin = tauri_plugin_window_state::Builder::default().with_state_flags(flags);
  let mut app = tauri::Builder::default();

  app = app
    .plugin(window_state_plugin.build())
    .plugin(tauri_nspanel::init());

  app
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      window.remove_shadow();

      // FIXME: this is crashing the app for some reason?
      window.set_float_panel(HIGHER_LEVEL_THAN_LEAGUE);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
