import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [details, setDetails] = useState("");
  const [id, setId] = useState("");
  const [plaintext, setPlaintext] = useState("");

  const runSetup = async () => {
    const result = await invoke("setup_command");
    setDetails(result);
  };

  const runExtract = async () => {
    const result = await invoke("extract_command", { id });
    setDetails(result);
  };

  const runEncrypt = async () => {
    const result = await invoke("encrypt_command", { id, plaintext });
    setDetails(result);
  };

  const runDecrypt = async () => {
    const result = await invoke("decrypt_command");
    setDetails(result);
  };

  return (
    <div className="container">
      <div className="left-panel">
        <h2>KR-IBE Demo</h2>

        <button onClick={runSetup} className="action-button">🔵 Setup</button>

        <input
          type="text"
          placeholder="Enter ID for Extract"
          value={id}
          onChange={(e) => setId(e.target.value)}
          className="input-box"
        />
        <button onClick={runExtract} className="action-button">🟢 Extract</button>

        <input
          type="text"
          placeholder="Enter Plaintext to Encrypt"
          value={plaintext}
          onChange={(e) => setPlaintext(e.target.value)}
          className="input-box"
        />
        <button onClick={runEncrypt} className="action-button">🟣 Encrypt</button>

        <button onClick={runDecrypt} className="action-button">🟠 Decrypt</button>
      </div>

      <div className="right-panel">
        <h3>Details Output:</h3>
        <pre>{details}</pre>
      </div>
    </div>
  );
}

export default App;
