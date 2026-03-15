#[cfg(debug_assertions)]
use tauri::Manager;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillFile {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveredSkill {
    pub id: String,
    pub name: String,
    pub path: String,
    pub tool: String,
    pub files: Vec<SkillFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub exists: bool,
}

#[tauri::command]
fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find home directory".to_string())
}

#[tauri::command]
fn scan_skills_for_tool(
    tool_id: String,
    base_path: String,
) -> Result<Vec<DiscoveredSkill>, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let tool_path = base_path.replace("~", &home.to_string_lossy());

    let path = PathBuf::from(&tool_path);

    if !path.exists() {
        return Ok(vec![]);
    }

    let mut skills = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let skill_name = entry_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                let mut files = Vec::new();

                if let Ok(file_entries) = fs::read_dir(&entry_path) {
                    for file_entry in file_entries.flatten() {
                        let file_path = file_entry.path();
                        files.push(SkillFile {
                            name: file_path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            path: file_path.to_string_lossy().to_string(),
                            is_dir: file_path.is_dir(),
                        });
                    }
                }

                skills.push(DiscoveredSkill {
                    id: format!("{}-{}", tool_id, skill_name),
                    name: skill_name,
                    path: entry_path.to_string_lossy().to_string(),
                    tool: tool_id.clone(),
                    files,
                });
            }
        }
    }

    Ok(skills)
}

#[tauri::command]
fn check_tool_path(tool_id: String, default_path: String) -> ToolInfo {
    let home = dirs::home_dir().unwrap_or_default();
    let path = default_path.replace("~", &home.to_string_lossy());
    let exists = PathBuf::from(&path).exists();

    ToolInfo {
        id: tool_id.clone(),
        name: tool_id,
        path,
        exists,
    }
}

#[tauri::command]
fn read_skill_file(file_path: String) -> Result<String, String> {
    fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn create_skill_directory(tool_path: String, skill_name: String) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let full_path = tool_path.replace("~", &home.to_string_lossy());

    let skill_dir = PathBuf::from(&full_path).join(&skill_name);

    fs::create_dir_all(&skill_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    let skill_md = skill_dir.join("SKILL.md");
    fs::write(
        &skill_md,
        format!("# {}\n\nAdd your skill description here.", skill_name),
    )
    .map_err(|e| format!("Failed to create SKILL.md: {}", e))?;

    Ok(skill_dir.to_string_lossy().to_string())
}

#[tauri::command]
fn copy_skill_to_tool(source_path: String, target_tool_path: String) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let target = target_tool_path.replace("~", &home.to_string_lossy());

    let source = PathBuf::from(&source_path);
    let skill_name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| "Invalid source path".to_string())?;

    let dest = PathBuf::from(&target).join(&skill_name);

    if dest.exists() {
        return Err("Skill already exists in target location".to_string());
    }

    fs::create_dir_all(&dest).map_err(|e| format!("Failed to create directory: {}", e))?;

    copy_dir_all(&source, &dest).map_err(|e| format!("Failed to copy skill: {}", e))?;

    Ok(dest.to_string_lossy().to_string())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_home_dir,
            scan_skills_for_tool,
            check_tool_path,
            read_skill_file,
            create_skill_directory,
            copy_skill_to_tool,
        ])
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
