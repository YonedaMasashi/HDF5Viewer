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

function DynamicDF() {
  const [columns, setColumns] = useState<any[]>([]);
  const [rows, setRows] = useState<DataFrameRow[]>([]);

  useEffect(() => {
    async function fetchData() {
      const result = await invoke<string>('get_dataframe_dynamic');

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
    }
    fetchData();

  }, []);

  return (
    <Container style={{ height: 400, width: '100%' }}>
        <DataGrid rows={rows} columns={columns} pageSize={5} checkboxSelection />
    </Container>
  );
}

export default DynamicDF;