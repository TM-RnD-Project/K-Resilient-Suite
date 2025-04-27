import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [details, setDetails] = useState("");

  const runSetup = async () => {
    const result = await invoke("setup_command");
    setDetails(result);
  };

  const runExtract = async () => {
    const id = document.getElementById("id").value;
    const result = await invoke("extract_command", { id });
    setDetails(result);
  };

  const runEncrypt = async () => {
    const id = document.getElementById("id").value;
    const plaintext = document.getElementById("plaintext").value;
    const result = await invoke("encrypt_command", { id, plaintext });
    setDetails(result);
  };

  const runDecrypt = async () => {
    const result = await invoke("decrypt_command");
    setDetails(result);
  };

  return (
    <div style={{ display: "flex", padding: "20px" }}>
      <div style={{ width: "40%", marginRight: "20px" }}>
        <button onClick={runSetup}>Setup</button><br/><br/>
        <input id="id" placeholder="Enter ID" /><br/><br/>
        <input id="plaintext" placeholder="Enter Plaintext" /><br/><br/>
        <button onClick={runExtract}>Extract</button><br/><br/>
        <button onClick={runEncrypt}>Encrypt</button><br/><br/>
        <button onClick={runDecrypt}>Decrypt</button><br/><br/>
      </div>
      <div style={{ width: "60%", background: "#f0f0f0", padding: "10px" }}>
        <pre>{details}</pre>
      </div>
    </div>
  );
}

export default App;
