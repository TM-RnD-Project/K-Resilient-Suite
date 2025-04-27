import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [details, setDetails] = useState("");
  const [id, setId] = useState("");
  const [plaintext, setPlaintext] = useState("");
  const [keyword, setKeyword] = useState("");
  const [scheme, setScheme] = useState("kr-ibe");

  // -------------- Commands --------------

  const runSetup = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_setup"));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_setup"));
    else if (scheme === "kr-peks") setDetails(await invoke("kr_peks_setup"));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_setup"));
  };

  const runExtractOrKeygen = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_extract", { id }));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_extract", { id }));
    else if (scheme === "kr-peks") setDetails(await invoke("kr_peks_extract", { id }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_keygen"));
  };

  const runEncryptOrSign = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_encrypt", { id, plaintext }));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_sign", { id }));
    else if (scheme === "kr-peks") setDetails(await invoke("kr_peks_encrypt", { id, plaintext }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_encrypt", { keyword }));
  };

  const runDecryptOrVerify = async () => {
    if (scheme === "kr-ibe") setDetails(await invoke("kr_ibe_decrypt"));
    else if (scheme === "kr-ibi") setDetails(await invoke("kr_ibi_verify", { id }));
  };

  const runTrapdoor = async () => {
    if (scheme === "kr-peks") setDetails(await invoke("kr_peks_trapdoor", { id }));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_trapdoor", { keyword }));
  };

  const runTest = async () => {
    if (scheme === "kr-peks") setDetails(await invoke("kr_peks_test"));
    else if (scheme === "kr-paeks") setDetails(await invoke("kr_paeks_test"));
  };

  // -------------- UI --------------
  return (
    <div className="container">
      <div className="left-panel">
        <h2>K-Resilient Suite Demo</h2>

        {/* Scheme Selector */}
        <select value={scheme} onChange={(e) => setScheme(e.target.value)} className="input-box">
          <option value="kr-ibe">KR-IBE</option>
          <option value="kr-ibi">KR-IBI</option>
          <option value="kr-peks">KR-PEKS</option>
          <option value="kr-paeks">KR-PAEKS</option>
        </select>

        {/* Setup */}
        <button onClick={runSetup} className="action-button">🔵 Setup</button>

        {/* Input Fields */}
        {(scheme === "kr-ibe" || scheme === "kr-ibi") && (
          <input
            type="text"
            placeholder="Enter ID"
            value={id}
            onChange={(e) => setId(e.target.value)}
            className="input-box"
          />
        )}

        {(scheme === "kr-ibe" || scheme === "kr-peks") && (
          <input
            type="text"
            placeholder="Enter Plaintext"
            value={plaintext}
            onChange={(e) => setPlaintext(e.target.value)}
            className="input-box"
          />
        )}

        {(scheme === "kr-peks" || scheme === "kr-paeks") && (
          <input
            type="text"
            placeholder="Enter Keyword"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            className="input-box"
          />
        )}

        {/* Main Action Buttons */}
        <button onClick={runExtractOrKeygen} className="action-button">🟢 {scheme === "kr-paeks" ? "Keygen" : "Extract"}</button>
        <button onClick={runEncryptOrSign} className="action-button">🟣 {scheme === "kr-ibi" ? "Sign" : "Encrypt"}</button>

        {/* Decrypt or Verify */}
        {(scheme === "kr-ibe" || scheme === "kr-ibi") && (
          <button onClick={runDecryptOrVerify} className="action-button">🟠 {scheme === "kr-ibi" ? "Verify" : "Decrypt"}</button>
        )}

        {/* Trapdoor (PEKS, PAEKS) */}
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



// function App() {
//   const [details, setDetails] = useState("");
//   const [id, setId] = useState("");
//   const [plaintext, setPlaintext] = useState("");

//   const runSetup = async () => {
//     const result = await invoke("setup_command");
//     setDetails(result);
//   };

//   const runExtract = async () => {
//     const result = await invoke("extract_command", { id });
//     setDetails(result);
//   };

//   const runEncrypt = async () => {
//     const result = await invoke("encrypt_command", { id, plaintext });
//     setDetails(result);
//   };

//   const runDecrypt = async () => {
//     const result = await invoke("decrypt_command");
//     setDetails(result);
//   };

//   return (
//     <div className="container">
//       <div className="left-panel">
//         <h2>KR-IBE Demo</h2>

//         <button onClick={runSetup} className="action-button">🔵 Setup</button>

//         <input
//           type="text"
//           placeholder="Enter ID for Extract"
//           value={id}
//           onChange={(e) => setId(e.target.value)}
//           className="input-box"
//         />
//         <button onClick={runExtract} className="action-button">🟢 Extract</button>

//         <input
//           type="text"
//           placeholder="Enter Plaintext to Encrypt"
//           value={plaintext}
//           onChange={(e) => setPlaintext(e.target.value)}
//           className="input-box"
//         />
//         <button onClick={runEncrypt} className="action-button">🟣 Encrypt</button>

//         <button onClick={runDecrypt} className="action-button">🟠 Decrypt</button>
//       </div>

//       <div className="right-panel">
//         <h3>Details Output:</h3>
//         <pre>{details}</pre>
//       </div>
//     </div>
//   );
// }

// export default App;
