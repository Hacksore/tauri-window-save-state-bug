pub mod macos {
  use cocoa::appkit::NSWindowCollectionBehavior;
  use tauri::{Runtime, Window};
  use tauri_nspanel::WindowExt;

  pub trait WindowExtMacos {
    fn set_float_panel(&self, level: i32);
  }

  impl<R: Runtime> WindowExtMacos for Window<R> {
    fn set_float_panel(&self, level: i32) {
      let panel = self.to_panel().unwrap();

      panel.set_level(level);

      #[allow(non_upper_case_globals)]
      const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

      #[allow(non_upper_case_globals)]
      const NSResizableWindowMask: i32 = 1 << 3;

      panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);

      panel.set_collection_behaviour(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
          | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
      );
    }
  }
}
