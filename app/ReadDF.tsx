"use client"; // これを追加してクライアントコンポーネントとして指定

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

// DataFrameRow 型を定義
interface DataFrameRow {
    column1: number;
    column2: string;
    column3: string;
  }
  
const App: React.FC = () => {
  const [filePath, setFilePath] = useState<string>("");
  const [dataFrame, setDataFrame] = useState<DataFrameRow[]>([]);

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      const result = await invoke<DataFrameRow[]>('read_data_frame', { filePath });
      setDataFrame(result);
    } catch (error) {
      console.error('Error invoking Rust command:', error);
    }
  };

  return (
    <div className="ReadDF">
      <h1>DataGrid Example</h1>
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
      <table>
        <thead>
          <tr>
            <th>Column 1</th>
            <th>Column 2</th>
            <th>Column 3</th>
          </tr>
        </thead>
        <tbody>
          {dataFrame.map((row, index) => (
            <tr key={index}>
              <td>{row.column1}</td>
              <td>{row.column2}</td>
              <td>{row.column3}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
