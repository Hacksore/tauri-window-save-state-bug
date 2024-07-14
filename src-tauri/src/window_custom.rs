pub mod macos {
  use tauri::{Runtime, Window};
  use tauri_nspanel::WindowExt;

  pub trait WindowExtMacos {
    fn set_level(&self, level: i32);
  }

  impl<R: Runtime> WindowExtMacos for Window<R> {
    fn set_level(&self, level: i32) {
      let panel = self.to_panel().expect("failed to create NSPanel");

      panel.set_level(level);

    }
  }
}
