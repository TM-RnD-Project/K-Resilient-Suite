import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function Upload({ user }) {
  const [receiver, setReceiver] = useState("");
  const [message, setMessage] = useState("");
  const [keyword, setKeyword] = useState("");
  const [status, setStatus] = useState("");

  const handleUpload = async () => {
    if (!receiver.trim() || !message.trim() || !keyword.trim()) {
      setStatus("Please fill in receiver, message, and keyword.");
      return;
    }

    try {
      setStatus("Encrypting and uploading...");

      await invoke("upload_file", {
        sender: user,
        receiver,
        msg: message,
        keyword,
      });

      setStatus("Upload successful.");
      setReceiver("");
      setMessage("");
      setKeyword("");
    } catch (error) {
      console.error(error);
      setStatus(`Upload failed: ${error}`);
    }
  };

  return (
    <div className="section-card">
      <h2>Upload Secure Message</h2>

      <input
        type="text"
        placeholder="Receiver ID"
        value={receiver}
        onChange={(e) => setReceiver(e.target.value)}
      />

      <textarea
        placeholder="Message"
        value={message}
        onChange={(e) => setMessage(e.target.value)}
      />

      <input
        type="text"
        placeholder="Keyword"
        value={keyword}
        onChange={(e) => setKeyword(e.target.value)}
      />

      <button onClick={handleUpload}>Upload</button>

      <p className="status-text">{status}</p>
    </div>
  );
}