import React from 'react';
import { dialog } from "@tauri-apps/api";

type FileInputProps = {
  filePath: string;
  onFilePathChange: (newFilePath: string) => void;
};

const FileInput: React.FC<FileInputProps> = ({ filePath, onFilePathChange }) => {
  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    console.log("FileInput:" + filePath);
    onFilePathChange(event.target.value);
  };

  return (
    <div>
      <input type="text" value={filePath} onChange={handleInputChange} placeholder="Enter file path" />
    </div>
  );
};

export default FileInput;
