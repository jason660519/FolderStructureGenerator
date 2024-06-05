import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const LocalDocumentsStructureGenerator = () => {
  const [folderPath, setFolderPath] = useState("");
  const [localDocumentsStructure, setLocalDocumentsStructure] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSaving, setIsSaving] = useState(false);

  const handleGenerateStructure = async () => {
    setIsGenerating(true);
    try {
      const result = await invoke<string>("generate_local_documents_structure", { path: folderPath });
      setLocalDocumentsStructure(result);
    } catch (err) {
      console.error("Error generating local documents structure:", err);
      alert(`Error generating local documents structure: ${err}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleSaveStructureToFile = async () => {
    setIsSaving(true);
    try {
      await invoke("generate_folder_structure", { path: folderPath });
      alert("Tree structure description file successfully generated!");
    } catch (err) {
      console.error("Error generating folder structure file:", err);
      alert(`Error generating folder structure file: ${err}`);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="container_LocalDocumentsStructure">
      <h1>Generate Local Documents Structure</h1>
      <div className="input-buttons-container">
        <input
          value={folderPath}
          onChange={(e) => setFolderPath(e.currentTarget.value)}
          placeholder="輸入文件夾路徑"
        />
        <div className="buttons-container">
          <button onClick={handleGenerateStructure} disabled={isGenerating}>
            在網頁生成本資料夾內所有文件的結構
          </button>
          <button onClick={handleSaveStructureToFile} disabled={isSaving || !folderPath}>
            在該資料夾底下生成資料夾內所有文件的結構txt檔
          </button>
        </div>
      </div>
      <hr />
      {localDocumentsStructure && (
        <div className="structure-container">
          <pre>{localDocumentsStructure}</pre>
        </div>
      )}
    </div>
  );
};

export default LocalDocumentsStructureGenerator;
