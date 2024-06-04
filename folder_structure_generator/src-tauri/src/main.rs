#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
    )]
    
    
    use std::{
        fs::{self, File},
        io::Write,
        path::{PathBuf},
    };
    
    #[tauri::command]
    async fn generate_folder_structure(path: String) -> Result<String, String> {
        // 將輸入的路徑轉化為PathBuf對象
        let path_buf = PathBuf::from(path.clone());
        // 檢查是否存在該路徑
        if !path_buf.exists() {
            return Err(format!("Path does not exist: {}", path));
        }
    
        let result = get_folder_structure(&path_buf)?;
        
        // 生成的檔案路徑
        let file_path = path_buf.join("資料夾結構說明.txt");
    
        // 創建檔案並寫入結構
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        write!(file, "{}", result).map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        Ok(result)
    }
    
    #[tauri::command]
    async fn generate_file_contents(path: String) -> Result<String, String> {
        // 這裡的邏輯根據你的需求可進行調整
        let path_buf = PathBuf::from(path.clone());
        if !path_buf.exists() {
            return Err(format!("Path does not exist: {}", path));
        }
    
        let contents = "這裡是文件內容的生成示例。";
        let file_path = path_buf.join("資料內容說明.txt");
        // 創建檔案並寫入內容
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        write!(file, "{}", contents).map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        Ok(contents.to_string())
    }
    
    // 递归获取文件夹结构
    fn get_folder_structure(path: &PathBuf) -> Result<String, String> {
        let mut result = String::new();
        let paths = fs::read_dir(path).map_err(|e| e.to_string())?;
        for path in paths {
            let path = path.map_err(|e| e.to_string())?.path();
            result.push_str(&format!("{}\n", path.display()));
            if path.is_dir() {
                result.push_str(&get_folder_structure(&path)?);
            }
        }
        Ok(result)
    }
    
    fn main() {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                generate_folder_structure,
                generate_file_contents
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
