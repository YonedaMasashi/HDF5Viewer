"use client"

import Image from "next/image";
import Greet from './greet';
import SampleDF from './SampleDF';
import FilePath from './FilePath';
import ReadDF from './ReadDF';
import DynamicDF2 from "./DynamicDF2";
import Sidebar from "./Sidebar";
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
      {/*<Sidebar />*/}
      <SidebarFromHdf5 filePath={inputFilePath} onKeyChange={handleSelectIdsChange} />

      <main>
        {/* <Greet /> */}
        {/*<FilePath />
        <SampleDF />
        <ReadDF /> 
        <DynamicDF2 /> */}

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
