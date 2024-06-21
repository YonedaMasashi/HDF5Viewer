"use client";

import React, { useState, useEffect } from 'react';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view';
import { styled } from '@mui/system';
import { Hdf5Node } from './types';
import { invoke } from '@tauri-apps/api/tauri';
import { Label } from '@mui/icons-material';
import { Box } from '@mui/material';
  
type HDF5ContentsProps = {
  filePath: string;
  fullKey: string;
};


const HDF5Contents: React.FC<HDF5ContentsProps> = ({ filePath, fullKey }) => {

  const [contents, setContents] = useState<string>("");

  console.log("HDF5Contents 1: " + filePath);
  console.log("HDF5Contents 1: " + fullKey);

  useEffect(() => {
    async function fetchData() {
      console.log("HDF5Contents 2: " + filePath);
      invoke<string>('read_hdf5_data', { filePath, fullKey })
          .then((response) => setContents(response))
          .catch((error) => setContents(error));
      
      console.log("HDF5Contents 3: " + contents);
    }
    fetchData();

  }, [filePath, fullKey]);

//   const renderContents = () => (
//       <Box>
//         {contents}
//       </Box>
//   );

  return (
    <div>
        <Box>
        {contents}
        </Box>
    </div>
    );
  
  };
  
  export default HDF5Contents;