"use client";

import React from 'react';
import { SimpleTreeView } from '@mui/x-tree-view/SimpleTreeView';
import { TreeItem } from '@mui/x-tree-view';
import { styled } from '@mui/system';
  
const Sidebar: React.FC = () => {
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
  };
  
  export default Sidebar;