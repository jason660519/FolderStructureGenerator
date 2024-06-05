import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import DocumentsInLocalFolderStructureGenerator from "./components/DocumentsInLocalFolderStructureGenerator"; 
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <Router>
    <Routes>
      <Route path="/" element={<App />} />
      <Route path="/structure" element={<DocumentsInLocalFolderStructureGenerator />} />
    </Routes>
  </Router>
);