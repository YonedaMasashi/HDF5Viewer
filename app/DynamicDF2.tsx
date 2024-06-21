"use client";

import React, { useEffect, useState } from 'react';
import { DataGrid } from '@mui/x-data-grid';
import { Container } from '@mui/material';
import { invoke } from '@tauri-apps/api/tauri';

interface DataFrameSchema {
  name: string;
  dtype: string;
}

interface DataFrameRow {
  [key: string]: any;
}

interface DataFrameResponse {
  schema: DataFrameSchema[];
  data: DataFrameRow[];
}

function DynamicDF2() {
  const [columns, setColumns] = useState<any[]>([]);
  const [rows, setRows] = useState<DataFrameRow[]>([]);
  const [filePath, setFilePath] = useState<string>("");

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      const result = await invoke<string>('get_dataframe_dynamic', { filePath });

      console.log(result);

      const data: DataFrameResponse = JSON.parse(result);

      // { field: 'id', headerName: 'ID', width: 90 },

      const cols = data.schema.map((col) => ({
        field: col.name,
        headerName: col.name, 
        width: 90 
      }));
      
      console.log(cols);

      setColumns(cols);
      setRows(data.data);
    } catch (error) {
      console.error('Error invoking Rust command:', error);
    }
  };

  return (
    <div className="ReadDF">
      <form onSubmit={handleSubmit} style={{ width: '100%' }}>
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
      <Container style={{ height: 400, width: '100%' }}>
          <DataGrid rows={rows} columns={columns} pageSize={5} checkboxSelection />
      </Container>
    </div>
  );
}

export default DynamicDF2;