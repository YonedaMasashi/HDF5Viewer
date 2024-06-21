"use client"; // これを追加してクライアントコンポーネントとして指定

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const InputFilePath: React.FC = () => {
  const [filePath, setFilePath] = useState<string>("");

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      await invoke('process_file_path', { filePath });
      alert("File path sent to Rust!");
    } catch (error) {
      console.error('Error invoking Rust command:', error);
    }
  };

  return (
    <div className="App">
      <h1>File Path Input Form</h1>
      <form onSubmit={handleSubmit}>
        <label>
          File Path:
          <input
            type="text"
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
          />
        </label>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}

export default InputFilePath;
