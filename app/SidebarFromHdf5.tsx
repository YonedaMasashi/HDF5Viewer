"use client";

import React, { useState, useEffect, SyntheticEvent } from 'react';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view';
import { Hdf5Node } from './types';
import { invoke } from '@tauri-apps/api/tauri';

import "./globals.css";


type SidebarFromHdf5Props = {
  filePath: string;
  onKeyChange: (newKey: string[]) => void;
};


const SidebarFromHdf5: React.FC<SidebarFromHdf5Props> = ({ filePath, onKeyChange }) => {
  const [treeData, setTreeData] = useState<Hdf5Node | null>(null);
  const [selectedItems, setSelectedItems] = React.useState<string[]>([]);

  console.log("SidebarFromHdf5 1: " + filePath);

  const handleNodeSelect = (event: SyntheticEvent, itemIds: string[]) => {
    setSelectedItems(itemIds);
    onKeyChange(itemIds);
  };


  useEffect(() => {
    async function fetchData() {
      console.log("SidebarFromHdf5 2: " + filePath);
      invoke<string>('get_hdf5_keys', { filePath })
          .then((response) => {
            console.log("SidebarFromHdf5 response: " + response);
              setTreeData(JSON.parse(response));
            })
          .catch((error) => console.error(error));
      
      console.log("SidebarFromHdf5 3 " + treeData);
    }
    fetchData();

  }, [filePath]);

  const renderTree = (nodes: Hdf5Node) => (
      <TreeItem key={nodes.name} id={nodes.full_key} itemId={nodes.full_key} label={nodes.name}  >
          {Array.isArray(nodes.children) ? nodes.children.map((node) => renderTree(node)) : null}
      </TreeItem>
  );

  return (
    <div>
        {treeData && (
            <SimpleTreeView
                aria-label="hdf5 tree"
                selectedItems={selectedItems}
                onSelectedItemsChange={handleNodeSelect}
                style={{ width: '300px' }} 
            >
                {renderTree(treeData)}
            </SimpleTreeView>
        )}
    </div>
  );
};
  
export default SidebarFromHdf5;