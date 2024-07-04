"use client";

import React, { useState, useEffect } from 'react';
import { Box } from '@mui/material';
  
type PropertySideBarProps = {
  propertyData: string;
};

const PropertySideBar: React.FC<PropertySideBarProps> = ({ propertyData }) => {
    
  return (
    <div  className="contents">
        <Box height={300}>
          {propertyData}
        </Box>
    </div>
    );
  };
  
  export default PropertySideBar;