"use client"

import sidebar_css from "./styles/sidebar.module.css";
import contents_css from "./styles/contents.module.css";

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
      <div className={sidebar_css.sidebar}>
      {/*<div style={{ width: '200px' }}>*/}
        <SidebarFromHdf5 filePath={inputFilePath} onKeyChange={handleSelectIdsChange} />
      </div>

      <main>
        <div>
          <FileInput
            filePath={inputFilePath}
            onFilePathChange={handleFilePathChange}
          />
        </div>

        <div className={contents_css.contents}>
          <HDF5Contents
            filePath={inputFilePath}
            fullKey={selectIds}
          />
        </div>

      </main>
    </div>
  );
}
