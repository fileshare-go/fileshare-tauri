import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useState } from "react";

function App() {
  const [file, setFile] = useState("");

  async function upload(path: string) {
    await invoke("upload", { path });
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>
      <h2>Welcom Back Dude!</h2>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          upload(file);
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setFile(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Upload!</button>
      </form>
    </main>
  );
}

export default App;
