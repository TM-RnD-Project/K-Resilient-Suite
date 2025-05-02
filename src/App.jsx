import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [details, setDetails] = useState("");
  const [id, setId] = useState("");
  const [plaintext, setPlaintext] = useState("");
  const [keyword, setKeyword] = useState("");
  const [scheme, setScheme] = useState("kr-ibe");
  const [extracted, setExtracted] = useState(false);
  const [k, setK] = useState();

  // -------------- Commands --------------

  const runSetup = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_setup", { k }));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_setup", { k }));
    else if (scheme === "kr-peks") setDetails(await invoke("kr_peks_setup", { k }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_setup", { k }));
  };

  const runExtractOrKeygen = async () => {
    if (scheme === "kr-ibe") {
      setDetails(await invoke("kr_ibe_extract", { id }));
      setExtracted(true);
    } else if (scheme === "kr-ibi") {
      setDetails(await invoke("kr_ibi_extract", { id }));
    } else if (scheme === "kr-peks") {
      setDetails(await invoke("kr_peks_keygen", { id }));
    } else if (scheme === "kr-paeks") {
      setDetails(await invoke("kr_paeks_keygen"));
    }
  };

  const runEncryptOrSign = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_encrypt", { id, plaintext }));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_sign", { id }));
    else if (scheme === "kr-peks") setDetails(await invoke("kr_peks_encrypt", { keyword }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_encrypt", { keyword }));
  };

  const runDecryptOrVerify = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_decrypt"));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_verify", { id }));
  };

  const runTrapdoor = async () => {
    if (scheme === "kr-peks") setDetails(await invoke("kr_peks_trapdoor", { keyword }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_trapdoor", { keyword }));
  };

  const runTest = async () => {
    if (scheme === "kr-peks") setDetails(await invoke("kr_peks_test"));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_test"));
  };

  const handleSchemeChange = (e) => {
    setScheme(e.target.value);
    setExtracted(false);
    setId("");
    setPlaintext("");
    setKeyword("");
  };

  // -------------- UI --------------
  return (
    <div className="container">
      <div className="left-panel">
        <h2>K-Resilient Suite Demo</h2>

        <label className="label">Please select a scheme to test:</label>
        <select value={scheme} onChange={handleSchemeChange} className="input-box">
          <option value="kr-ibe">KR-IBE</option>
          <option value="kr-ibi">KR-IBI</option>
          <option value="kr-peks">KR-PEKS</option>
          <option value="kr-paeks">KR-PAEKS</option>
        </select>

        <label className="label">Please input the value of k:</label>
        <input
          type="number"
          placeholder="e.g., 20"
          value={k}
          onChange={(e) => setK(parseInt(e.target.value))}
          className="input-box"
        />

        <button onClick={runSetup} className="action-button">🔵 Setup</button>

        {(scheme === "kr-ibe" || scheme === "kr-ibi") && (
          <>
            <label className="label">Please input the ID:</label>
            <input
              type="text"
              placeholder="Enter ID"
              value={id}
              onChange={(e) => setId(e.target.value)}
              className="input-box"
            />
          </>
        )}

        <button onClick={runExtractOrKeygen} className="action-button">
          🟢 {(scheme === "kr-paeks" || scheme === "kr-peks") ? "Keygen" : "Extract"}
        </button>

        {(scheme === "kr-ibe" && extracted) && (
          <>
            <label className="label">Please input the plaintext to encrypt:</label>
            <input
              type="text"
              placeholder="Enter Plaintext"
              value={plaintext}
              onChange={(e) => setPlaintext(e.target.value)}
              className="input-box"
            />
          </>
        )}

        {(scheme === "kr-paeks" || scheme === "kr-peks") && (
          <>
            <label className="label">Please input the keyword:</label>
            <input
              type="text"
              placeholder="Enter Keyword"
              value={keyword}
              onChange={(e) => setKeyword(e.target.value)}
              className="input-box"
            />
          </>
        )}

        <button onClick={runEncryptOrSign} className="action-button">
          🟣 {scheme === "kr-ibi"
            ? "Prove"
            : scheme === "kr-peks"
            ? "PEKS"
            : scheme === "kr-paeks"
            ? "PAEKS"
            : "Encrypt"}
        </button>

        {(scheme === "kr-ibe" || scheme === "kr-ibi") && (
          <button onClick={runDecryptOrVerify} className="action-button">
            🟠 {scheme === "kr-ibi" ? "Verify" : "Decrypt"}
          </button>
        )}

        {(scheme === "kr-peks" || scheme === "kr-paeks") && (
          <>
            <button onClick={runTrapdoor} className="action-button">🟠 Trapdoor</button>
            <button onClick={runTest} className="action-button">⚡ Test</button>
          </>
        )}
      </div>

      <div className="right-panel">
        <h3>Details Output:</h3>
        <pre>{details}</pre>
      </div>
    </div>
  );
}

export default App;
