"use client";

import { invoke } from '@tauri-apps/api/tauri';

import React, { useState, useEffect } from 'react';
import { Box } from '@mui/material';
  
type PropertySideBarProps = {
  filePath: string;
  fullKey: string | null;
};


interface Attribute {
  name: string;
  value: string;
}


async function readHDF5Attributes(filePath: string, fullKey: string | null): Promise<Attribute[]> {
  try {
    const attributes = await invoke('read_hdf5_attributes', { filePath, fullKey });
    return attributes as Attribute[];
  } catch (error) {
    console.error('Error reading HDF5 attributes:', error);
    return [];
  }
}


const PropertySideBar: React.FC<PropertySideBarProps> = ({ filePath, fullKey }) => {
  const [attributes, setAttributes] = useState<Attribute[]>([]);
  console.log("PropertySideBar 1: " + filePath);
  console.log("PropertySideBar 1: " + fullKey);
    
  useEffect(() => {
    const loadAttributes = async () => {
      const result = await readHDF5Attributes(filePath, fullKey);
      setAttributes(result);
      console.log("PropertySideBar 2: Atribute lenght : " + result.length)
    };
    loadAttributes();
  }, [filePath, fullKey]);

  return (
    <div  className="contents">
      <ul>
        {attributes.map((attr, index) => (
          <li key={index}>
            <strong>{attr.name}:</strong> {attr.value}
          </li>
        ))}
      </ul>
    </div>
    );
  };
  
  export default PropertySideBar;