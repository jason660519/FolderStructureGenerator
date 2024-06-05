// 此檔案包含用於生成文件夾結構和文件結構保存到本地文件的功能。

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

// 确保函数声明为 pub
#[tauri::command]
pub fn generate_local_documents_structure(path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist"));
    }
    get_folder_structure(&path_buf)
}

#[tauri::command]
pub fn generate_folder_structure(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist"));
    }
    let result = get_folder_structure(&path_buf)?;

    let file_path = path_buf.join("本資料夾內所有文件結構說明.txt");
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    write!(file, "{}", result).map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    Ok(())
}

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