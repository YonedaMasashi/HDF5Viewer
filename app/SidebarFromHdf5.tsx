"use client";

import React, { useState, useEffect, SyntheticEvent } from 'react';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view';
import { styled } from '@mui/system';
import { Hdf5Node } from './types';
import { invoke } from '@tauri-apps/api/tauri';
import { tree } from 'next/dist/build/templates/app-page';
  
type SidebarFromHdf5Props = {
  filePath: string;
  onKeyChange: (newKey: string[]) => void;
};


const SidebarFromHdf5: React.FC<SidebarFromHdf5Props> = ({ filePath, onKeyChange }) => {
  /* 以下の実装を改良して、rust で目次を作ってデータを取得する方法を検討する
      "data" の値を rust 側で作る */
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
      //const result = await invoke<string>('get_dataframe_dynamic', { filePath });
      
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
            >
                {renderTree(treeData)}
            </SimpleTreeView>
        )}
    </div>
);

  
    /*
    const [fileContent, setFileContent] = useState('');

    const renderTree = (nodes: any) => (
      <TreeItem key={nodes.id} itemId={nodes.id} label={nodes.name}>
        {Array.isArray(nodes.children)
          ? nodes.children.map((node: any) => renderTree(node))
          : null}
      </TreeItem>
    );

    const data = {
      id: 'root',
      name: 'Root',
      children: [
        {
          id: '1',
          name: 'Child - 1',
          children: [
            { id: '2', name: 'Child - 1.1' },
            { id: '3', name: 'Child - 1.2' },
          ],
        },
        {
          id: '4',
          name: 'Child - 2',
          children: [
            { id: '5', name: 'Child - 2.1' },
          ],
        },
      ],
    };
  
    return (
      <div>
        <SimpleTreeView>
          {renderTree(data)}
        </SimpleTreeView>
      </div>
    );
    */
  };
  
  export default SidebarFromHdf5;