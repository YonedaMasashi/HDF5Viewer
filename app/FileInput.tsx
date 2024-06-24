import React from 'react';

import fileinput_css from "./styles/fileinput.module.css";

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
      <input type="text" value={filePath} onChange={handleInputChange} placeholder="Enter hdf5 file path"  className={`${fileinput_css.file_path_bg_color} ${fileinput_css.file_path_default_setting}`}/>
    </div>
  );
};

export default FileInput;
