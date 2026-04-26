import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function Search({ user }) {
  const [keyword, setKeyword] = useState("");
  const [results, setResults] = useState([]);
  const [downloadedMessages, setDownloadedMessages] = useState([]);
  const [status, setStatus] = useState("");

  const handleSearch = async () => {
    if (!keyword.trim()) {
      setStatus("Please enter a keyword.");
      return;
    }

    try {
      setStatus("Searching encrypted data...");

      const indexes = await invoke("search_keyword", {
        user,
        keyword,
      });

      setResults(indexes);
      setStatus(`Found ${indexes.length} result(s).`);
    } catch (error) {
      console.error(error);
      setStatus(`Search failed: ${error}`);
    }
  };

  const handleDownload = async (index) => {
    try {
      const message = await invoke("download_file", {
        user,
        index,
      });

      setDownloadedMessages((prev) => [
        ...prev,
        { index, content: message },
      ]);
    } catch (error) {
      console.error(error);
      setStatus(`Download failed: ${error}`);
    }
  };

  return (
    <div className="section-card">
      <h2>Search Encrypted Messages</h2>

      <input
        type="text"
        placeholder="Keyword"
        value={keyword}
        onChange={(e) => setKeyword(e.target.value)}
      />

      <button onClick={handleSearch}>Search</button>

      <p className="status-text">{status}</p>

      <div className="results-block">
        <h3>Search Results</h3>
        {results.length === 0 ? (
          <p>No results yet.</p>
        ) : (
          <ul>
            {results.map((index) => (
              <li key={index}>
                Ciphertext #{index}
                <button onClick={() => handleDownload(index)}>
                  Download
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>

      <div className="results-block">
        <h3>Downloaded Messages</h3>
        {downloadedMessages.length === 0 ? (
          <p>No downloaded messages yet.</p>
        ) : (
          downloadedMessages.map((item, i) => (
            <div key={i} className="message-box">
              <strong>From result #{item.index}</strong>
              <p>{item.content}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}