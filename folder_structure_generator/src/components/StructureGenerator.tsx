import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const StructureGenerator = () => {
  const [folderPath, setFolderPath] = useState("");
  const [folderStructure, setFolderStructure] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);

  const handleGenerateStructure = async () => {
    setIsGenerating(true);
    try {
      // 使用 Tauri invoke 調用 Rust 函數生成資料夾樹狀結構
      const result = await invoke<string>("generate_folder_structure", { path: folderPath });
      setFolderStructure(result);
    } catch (err) {
      console.error(err);
    } finally {
      setIsGenerating(false);
    }
  };

  // 新增函數來處理生成檔案內容的邏輯
  const handleGenerateFileContents = async () => {
    setIsGenerating(true);
    try {
      // 此處調用另一個Tauri命令來生成檔案
      await invoke("generate_file_contents", { path: folderPath });
      alert("資料內容說明.txt 已在資料夾內生成！");
    } catch (err) {
      console.error(err);
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="container">
      <h1>Generate Folder Structure</h1>
      <input
        value={folderPath}
        onChange={(e) => setFolderPath(e.currentTarget.value)}
        placeholder="輸入文件夾路徑"
      />
      <button onClick={handleGenerateStructure} disabled={isGenerating}>
        在本網頁生成樹狀結構
      </button>
      {/* 判斷是否有生成的樹狀結構，如果有則顯示 */}
      {folderStructure && (
        <div>
          <pre>{folderStructure}</pre>
        </div>
      )}
      <button onClick={handleGenerateFileContents} disabled={isGenerating || !folderPath}>
        在該資料夾底下生成樹狀結構說明檔
      </button>
    </div>
  );
};

export default StructureGenerator;
