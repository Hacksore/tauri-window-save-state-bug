import { StateFlags, saveWindowState } from "tauri-plugin-window-state-api";
import "./App.css";
import { exit } from "@tauri-apps/api/process";

function App() {
  return (
    <div className="container">
      <button
        onClick={async (e) => {
          e.preventDefault();
          await saveWindowState(StateFlags.ALL);
          await exit();
        }}
      >
        Click me to close
      </button>
    </div>
  );
}

export default App;
