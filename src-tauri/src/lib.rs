#[cfg(debug_assertions)]
use tauri::Manager;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

// ============================================================
// Data Structures
// ============================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillFileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

/// A skill discovered from the file system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveredSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    /// "global-agent" or tool id like "opencode", "cursor", etc.
    pub source: String,
    pub source_label: String,
    pub files: Vec<SkillFileEntry>,
    /// Which tools currently have this skill (by checking existence)
    pub enabled_tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolConfig {
    pub id: String,
    pub name: String,
    pub skill_path: String,
    pub resolved_path: String,
    pub exists: bool,
    pub skill_count: usize,
    pub supports_global_agent: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub skills: Vec<DiscoveredSkill>,
    pub tools: Vec<ToolConfig>,
}

// ============================================================
// Tool definitions
// ============================================================

struct ToolDef {
    id: &'static str,
    name: &'static str,
    /// Relative to home dir, use forward slashes
    skill_path: &'static str,
    supports_global_agent: bool,
}

fn get_tool_definitions() -> Vec<ToolDef> {
    vec![
        ToolDef {
            id: "global-agent",
            name: "Global Agent",
            skill_path: ".agents/skills",
            supports_global_agent: false,
        },
        // ---- Tools that support global agent (.agents/skills) ----
        ToolDef {
            id: "amp",
            name: "Amp",
            skill_path: ".amp/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "cline",
            name: "Cline",
            skill_path: ".cline/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "codex",
            name: "Codex",
            skill_path: ".codex/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "cursor",
            name: "Cursor",
            skill_path: ".cursor/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "gemini-cli",
            name: "Gemini CLI",
            skill_path: ".gemini/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "github-copilot",
            name: "Github Copilot",
            skill_path: ".github-copilot/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "kimi-code",
            name: "Kimi Code CLI",
            skill_path: ".kimi-code/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "opencode",
            name: "OpenCode",
            skill_path: ".config/opencode/skills",
            supports_global_agent: true,
        },
        ToolDef {
            id: "warp",
            name: "Warp",
            skill_path: ".warp/skills",
            supports_global_agent: true,
        },
        // ---- Tools with independent skill directories only ----
        ToolDef {
            id: "claude-code",
            name: "Claude Code",
            skill_path: ".claude/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "trae",
            name: "Trae",
            skill_path: ".trae/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "windsurf",
            name: "Windsurf",
            skill_path: ".windsurf/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "augment",
            name: "Augment Code",
            skill_path: ".augment/skills",
            supports_global_agent: false,
        },
    ]
}

fn resolve_path(home: &PathBuf, relative: &str) -> PathBuf {
    // Split by '/' and join as path components for cross-platform compatibility
    let mut path = home.clone();
    for component in relative.split('/') {
        path = path.join(component);
    }
    path
}

// ============================================================
// Commands
// ============================================================

#[tauri::command]
fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find home directory".to_string())
}

/// Scan all skill directories and return a combined result
#[tauri::command]
fn scan_all_skills() -> Result<ScanResult, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let defs = get_tool_definitions();

    let mut all_skills: Vec<DiscoveredSkill> = Vec::new();
    let mut tools: Vec<ToolConfig> = Vec::new();

    // First pass: collect tool configs and their skill names
    let mut tool_skill_map: Vec<(String, String, Vec<String>)> = Vec::new(); // (tool_id, resolved_path, skill_names)

    for def in &defs {
        let resolved = resolve_path(&home, def.skill_path);
        let resolved_str = resolved.to_string_lossy().to_string();
        let exists = resolved.exists();

        let mut skill_names: Vec<String> = Vec::new();
        let mut skill_count = 0;

        if exists {
            if let Ok(entries) = fs::read_dir(&resolved) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        let name = entry_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        skill_names.push(name);
                        skill_count += 1;
                    }
                }
            }
        }

        tools.push(ToolConfig {
            id: def.id.to_string(),
            name: def.name.to_string(),
            skill_path: def.skill_path.to_string(),
            resolved_path: resolved_str.clone(),
            exists,
            skill_count,
            supports_global_agent: def.supports_global_agent,
        });

        tool_skill_map.push((def.id.to_string(), resolved_str, skill_names));
    }

    // Second pass: gather skills for each tool/source and figure out which other tools also have it
    for (tool_id, resolved_path, skill_names) in &tool_skill_map {
        let source_label = defs
            .iter()
            .find(|d| d.id == tool_id.as_str())
            .map(|d| d.name)
            .unwrap_or("Unknown")
            .to_string();

        for skill_name in skill_names {
            let skill_path = PathBuf::from(resolved_path).join(skill_name);

            // Read files in skill directory
            let files = read_skill_dir_files(&skill_path);

            // Read description from SKILL.md if possible
            let description = read_skill_description(&skill_path);

            // Find which tools have a skill with the same name
            let mut enabled_tools: Vec<String> = Vec::new();
            for (other_tool_id, _, other_skill_names) in &tool_skill_map {
                if other_tool_id != "global-agent" && other_skill_names.contains(skill_name) {
                    enabled_tools.push(other_tool_id.clone());
                }
            }

            all_skills.push(DiscoveredSkill {
                id: format!("{}-{}", tool_id, skill_name),
                name: skill_name.clone(),
                description,
                path: skill_path.to_string_lossy().to_string(),
                source: tool_id.clone(),
                source_label: source_label.clone(),
                files,
                enabled_tools,
            });
        }
    }

    Ok(ScanResult {
        skills: all_skills,
        tools,
    })
}

fn read_skill_dir_files(skill_path: &PathBuf) -> Vec<SkillFileEntry> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(skill_path) {
        for entry in entries.flatten() {
            let p = entry.path();
            files.push(SkillFileEntry {
                name: p
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default(),
                path: p.to_string_lossy().to_string(),
                is_dir: p.is_dir(),
            });
        }
    }
    files
}

fn read_skill_description(skill_path: &PathBuf) -> String {
    let skill_md = skill_path.join("SKILL.md");
    if skill_md.exists() {
        if let Ok(content) = fs::read_to_string(&skill_md) {
            // Parse YAML frontmatter for description
            if content.starts_with("---") {
                if let Some(end) = content[3..].find("---") {
                    let frontmatter = &content[3..3 + end];
                    for line in frontmatter.lines() {
                        let line = line.trim();
                        if line.starts_with("description:") {
                            return line["description:".len()..].trim().to_string();
                        }
                    }
                }
            }
            // Fallback: first non-empty, non-heading line
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                    return trimmed.to_string();
                }
            }
        }
    }
    String::new()
}

#[tauri::command]
fn read_skill_file(file_path: String) -> Result<String, String> {
    fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Enable a skill for a specific tool by creating a symlink
#[tauri::command]
fn enable_skill_for_tool(skill_path: String, tool_id: String) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("Unknown tool: {}", tool_id))?;

    let target_base = resolve_path(&home, def.skill_path);

    // Ensure the target base directory exists
    fs::create_dir_all(&target_base)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;

    let source = PathBuf::from(&skill_path);
    let skill_name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| "Invalid skill path".to_string())?;

    let dest = target_base.join(&skill_name);

    if dest.exists() {
        return Err("Skill already exists in target tool".to_string());
    }

    // Create symlink instead of copying
    create_symlink_dir(&source, &dest).map_err(|e| format!("Failed to create symlink: {}", e))?;

    Ok(dest.to_string_lossy().to_string())
}

/// Disable a skill for a specific tool by removing it
#[tauri::command]
fn disable_skill_for_tool(skill_name: String, tool_id: String) -> Result<(), String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("Unknown tool: {}", tool_id))?;

    let target = resolve_path(&home, def.skill_path).join(&skill_name);

    if !target.exists() {
        return Err("Skill not found in target tool".to_string());
    }

    fs::remove_dir_all(&target).map_err(|e| format!("Failed to remove skill: {}", e))?;

    Ok(())
}

/// Create a new skill in a specific tool (or global-agent)
#[tauri::command]
fn create_skill(
    tool_id: String,
    skill_name: String,
    description: String,
) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("Unknown tool: {}", tool_id))?;

    let base = resolve_path(&home, def.skill_path);
    fs::create_dir_all(&base).map_err(|e| format!("Failed to create base directory: {}", e))?;

    let skill_dir = base.join(&skill_name);

    if skill_dir.exists() {
        return Err("Skill already exists".to_string());
    }

    fs::create_dir_all(&skill_dir).map_err(|e| format!("Failed to create skill directory: {}", e))?;

    let content = format!(
        "---\nname: {}\ndescription: {}\n---\n\n# {}\n\n{}\n",
        skill_name, description, skill_name, description
    );

    let skill_md = skill_dir.join("SKILL.md");
    fs::write(&skill_md, content).map_err(|e| format!("Failed to create SKILL.md: {}", e))?;

    Ok(skill_dir.to_string_lossy().to_string())
}

/// Delete a skill entirely from its source
#[tauri::command]
fn delete_skill(skill_path: String) -> Result<(), String> {
    let path = PathBuf::from(&skill_path);
    if !path.exists() {
        return Err("Skill path does not exist".to_string());
    }

    fs::remove_dir_all(&path).map_err(|e| format!("Failed to delete skill: {}", e))?;

    Ok(())
}

/// Open a path in the system file explorer
#[tauri::command]
fn open_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open explorer: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open finder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }
    Ok(())
}

/// Create a directory symlink, handling cross-platform differences
fn create_symlink_dir(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    // Convert src to canonical absolute path for better symlink stability
    let src_absolute = fs::canonicalize(src)?;

    #[cfg(unix)]
    {
        unix_fs::symlink(&src_absolute, dst)?;
    }

    #[cfg(windows)]
    {
        // On Windows, use symlink_dir for directory symlinks
        windows_fs::symlink_dir(&src_absolute, dst)?;
    }

    Ok(())
}

// ============================================================
// App entry
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_home_dir,
            scan_all_skills,
            read_skill_file,
            enable_skill_for_tool,
            disable_skill_for_tool,
            create_skill,
            delete_skill,
            open_in_explorer,
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
