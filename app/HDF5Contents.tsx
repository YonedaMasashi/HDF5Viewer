"use client";

import React, { useState, useEffect } from 'react';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view';
import { styled } from '@mui/system';
import { Hdf5Node } from './types';
import { invoke } from '@tauri-apps/api/tauri';
import { Label } from '@mui/icons-material';
import { Box } from '@mui/material';
import { DataGrid } from '@mui/x-data-grid';
import { Container } from '@mui/material';
  
type HDF5ContentsProps = {
  filePath: string;
  fullKey: string;
};


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

const HDF5Contents: React.FC<HDF5ContentsProps> = ({ filePath, fullKey }) => {
  // 単一データ、配列時
  const [contents, setContents] = useState<string>("");
  
  // DataFrame時
  const [columns, setColumns] = useState<any[]>([]);
  const [rows, setRows] = useState<DataFrameRow[]>([]);

  // DataFrame 判定
  const [isDataFrame, setIsDataFrame] = useState<boolean>(false);

  useEffect(() => {
    async function fetchData() {
      console.log("HDF5Contents 1: " + filePath);
      console.log("HDF5Contents 1: " + fullKey);
      console.log("HDF5Contents 2: " + filePath);
      invoke<string>('read_hdf5_data', { filePath, fullKey })
        .then((response) => {
          console.log("HDF5Contents 3: " + response);
          setContents(response);
          console.log("HDF5Contents 4: " + contents);
            
          // DataFrame構築
          try {
            const data: DataFrameResponse = JSON.parse(response);
            const cols = data.schema.map((col) => ({
              field: col.name,
              headerName: col.name, 
              width: 90 
            }));
            setColumns(cols);
            setRows(data.data);
            setIsDataFrame(true);
          } catch (e) {
            setIsDataFrame(false);
          }
      
          console.log("isDataFrame :" + isDataFrame);
        })
        .catch((error) => setContents(error));
    }
    fetchData();


  }, [filePath, fullKey]);

  return (
    <div>
        { isDataFrame ? (
          <Container style={{ height: 400, width: '100%' }}>
            <DataGrid rows={rows} columns={columns} pageSize={5} checkboxSelection />
          </Container>
        ) : (
          <Box>
          {contents}
          </Box>
        )}
    </div>
    );
  
  };
  
  export default HDF5Contents;