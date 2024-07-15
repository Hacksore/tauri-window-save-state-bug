import { StateFlags, saveWindowState } from "tauri-plugin-window-state-api";
import "./App.css";
import { exit } from "@tauri-apps/api/process";

function App() {
  return (
    <div data-tauri-drag-region className="container">
      <button
        onClick={async (e) => {
          e.preventDefault();
          try {
            const sizeAndPos = StateFlags.POSITION | StateFlags.SIZE;
            await saveWindowState(sizeAndPos);
          } catch (e: any) {
            console.error(e);
          }
          await exit();
        }}
      >
        Click me to close
      </button>
    </div>
  );
}

export default App;
