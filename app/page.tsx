"use client"

import FileInput from "./FileInput";
import SidebarFromHdf5 from "./SidebarFromHdf5";
import HDF5Contents from "./HDF5Contents";

import React, { useState } from 'react';



export default function Home() {
  const [inputFilePath, setInputFilePath] = useState('');
  const [selectIds, setSelectIds] = useState<string[]>([]);

  const handleFilePathChange = (newFilePath: string) => {
    console.log("Paga.handleFilePathChange : " + newFilePath);
    setInputFilePath(newFilePath);
  };

  const handleSelectIdsChange = (newKey: string[]) => {
    console.log("Paga.handleSelectIdsChange : " + newKey);
    setSelectIds(newKey);
  };

  return (
    <div className="app">
      <SidebarFromHdf5 filePath={inputFilePath} onKeyChange={handleSelectIdsChange} />

      <main>
        <FileInput
          filePath={inputFilePath}
          onFilePathChange={handleFilePathChange}
        />

        <HDF5Contents
          filePath={inputFilePath}
          fullKey={selectIds}
        />

      </main>
    </div>
  );
}
