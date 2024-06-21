"use client";

import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

// DataFrameRow 型を定義
interface DataFrameRow {
  column1: string;
  column2: number;
}

const SampleDF: React.FC = () => {
  const [data, setData] = useState<DataFrameRow[]>([]);
  
  useEffect(() => {
    async function fetchData() {
      try {
        const result = await invoke<DataFrameRow[]>('get_dataframe');
        setData(result);
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }

    fetchData();
  }, []);

  return (
    <div className="SampleDF">
      <h1>DataGrid Example</h1>
      <table>
        <thead>
          <tr>
            <th>Column 1</th>
            <th>Column 2</th>
          </tr>
        </thead>
        <tbody>
          {data.map((row, index) => (
            <tr key={index}>
              <td>{row.column1}</td>
              <td>{row.column2}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default SampleDF;
