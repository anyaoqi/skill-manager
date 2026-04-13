#[cfg(debug_assertions)]
use tauri::Manager;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

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
            id: "qwen-code",
            name: "Qwen Code",
            skill_path: ".qwen/skills",
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
            id: "antigravity",
            name: "Antigravity",
            skill_path: ".gemini/antigravity/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "claude-code",
            name: "Claude Code",
            skill_path: ".claude/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "codebuddy",
            name: "CodeBuddy",
            skill_path: ".codebuddy/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "gemini-cli",
            name: "Gemini CLI",
            skill_path: ".gemini/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "github-copilot",
            name: "GitHub Copilot",
            skill_path: ".copilot/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "kiro",
            name: "Kiro",
            skill_path: ".kiro/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "qoder",
            name: "Qoder",
            skill_path: ".qoder/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "trae",
            name: "Trae",
            skill_path: ".trae/skills",
            supports_global_agent: false,
        },
        ToolDef {
            id: "trae-cn",
            name: "Trae CN",
            skill_path: ".trae-cn/skills",
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
        .ok_or_else(|| "无法找到用户主目录".to_string())
}

/// Scan all skill directories and return a combined result
#[tauri::command]
fn scan_all_skills() -> Result<ScanResult, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法找到用户主目录".to_string())?;
    let defs = get_tool_definitions();

    // Resolve global-agent path once for junction/symlink detection
    let global_agent_path = resolve_path(&home, ".agents/skills");
    let global_agent_canonical = fs::canonicalize(&global_agent_path).ok();

    let mut all_skills: Vec<DiscoveredSkill> = Vec::new();
    let mut tools: Vec<ToolConfig> = Vec::new();

    // First pass: collect tool configs and skill names.
    // Each entry: (tool_id, resolved_path, skill_names)
    // skill_names: names of skills that are *real* (not junctions pointing to global-agent)
    // For non-global-agent tools we also track junction-linked names separately.
    let mut tool_skill_map: Vec<(String, String, Vec<String>)> = Vec::new();

    // Also track: for each tool, which skill names are junctions pointing to global-agent
    let mut tool_junction_skills: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for def in &defs {
        let resolved = resolve_path(&home, def.skill_path);
        let resolved_str = resolved.to_string_lossy().to_string();
        let exists = resolved.exists();

        let mut skill_names: Vec<String> = Vec::new();
        let mut junction_names: Vec<String> = Vec::new();
        let mut skill_count = 0;

        if exists {
            if let Ok(entries) = fs::read_dir(&resolved) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    // Count both real dirs and junctions/symlinks that resolve to dirs
                    if entry_path.is_dir() {
                        let name = entry_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        // For non-global-agent tools, detect if this is a junction/symlink
                        // pointing into the global-agent directory
                        if def.id != "global-agent" {
                            if is_junction_to_global_agent(&entry_path, &global_agent_canonical) {
                                junction_names.push(name.clone());
                                skill_count += 1;
                                // Don't add to skill_names — handled via global-agent source
                                continue;
                            }
                        }

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

        tool_junction_skills.insert(def.id.to_string(), junction_names);
        tool_skill_map.push((def.id.to_string(), resolved_str, skill_names));
    }

    // Build a flat map: skill_name -> list of tool_ids that have it (via junction)
    // so we can attach enabled_tools info to global-agent skills
    let mut junction_tool_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for (tool_id, junction_names) in &tool_junction_skills {
        for name in junction_names {
            junction_tool_map
                .entry(name.clone())
                .or_default()
                .push(tool_id.clone());
        }
    }

    // Second pass: emit DiscoveredSkill entries
    for (tool_id, resolved_path, skill_names) in &tool_skill_map {
        let source_label = defs
            .iter()
            .find(|d| d.id == tool_id.as_str())
            .map(|d| d.name)
            .unwrap_or("Unknown")
            .to_string();

        for skill_name in skill_names {
            let skill_path = PathBuf::from(resolved_path).join(skill_name);

            let files = read_skill_dir_files(&skill_path);
            let description = read_skill_description(&skill_path);

            // Which tools have this skill (by name, excluding global-agent)
            let mut enabled_tools: Vec<String> = Vec::new();
            for (other_tool_id, _, other_skill_names) in &tool_skill_map {
                if other_tool_id != "global-agent" && other_skill_names.contains(skill_name) {
                    enabled_tools.push(other_tool_id.clone());
                }
            }
            // Also count tools that have it via junction
            if let Some(junction_tools) = junction_tool_map.get(skill_name) {
                for jt in junction_tools {
                    if !enabled_tools.contains(jt) {
                        enabled_tools.push(jt.clone());
                    }
                }
            }

            // For global-agent skills, also include tools that linked via junction
            // (they are already in enabled_tools via junction_tool_map above)

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

    //补全：对于只通过 junction 被发现（global-agent 目录不存在或未扫描到）的 skill，
    // 补充一条 source='global-agent' 的记录，保证分类正确。
    let global_agent_tool = tools.iter().find(|t| t.id == "global-agent");
    let global_agent_resolved = global_agent_tool
        .map(|t| t.resolved_path.clone())
        .unwrap_or_default();

    for (skill_name, linked_tools) in &junction_tool_map {
        // If already recorded from global-agent scan, skip
        if all_skills
            .iter()
            .any(|s| s.name == *skill_name && s.source == "global-agent")
        {
            continue;
        }
        // Find the actual path from any tool that has the junction
        // Use the junction target to get the real path
        let real_path = if !global_agent_resolved.is_empty() {
            PathBuf::from(&global_agent_resolved).join(skill_name)
        } else {
            // Fallback: use first tool's junction path as proxy
            if let Some(first_tool_id) = linked_tools.first() {
                let tool_path = tool_skill_map
                    .iter()
                    .find(|(id, _, _)| id == first_tool_id)
                    .map(|(_, p, _)| PathBuf::from(p).join(skill_name));
                // Try to read junction target
                if let Some(ref jpath) = tool_path {
                    #[cfg(windows)]
                    {
                        if let Ok(target) = junction::get_target(jpath) {
                            target
                        } else {
                            jpath.clone()
                        }
                    }
                    #[cfg(not(windows))]
                    {
                        if let Ok(target) = std::fs::read_link(jpath) {
                            target
                        } else {
                            jpath.clone()
                        }
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        };

        let files = read_skill_dir_files(&real_path);
        let description = read_skill_description(&real_path);

        all_skills.push(DiscoveredSkill {
            id: format!("global-agent-{}", skill_name),
            name: skill_name.clone(),
            description,
            path: real_path.to_string_lossy().to_string(),
            source: "global-agent".to_string(),
            source_label: "Global Agent".to_string(),
            files,
            enabled_tools: linked_tools.clone(),
        });
    }

    Ok(ScanResult {
        skills: all_skills,
        tools,
    })
}

/// Check whether `path` is a symlink/junction whose resolved target
/// lives inside the global-agent skills directory.
fn is_junction_to_global_agent(path: &PathBuf, global_agent_canonical: &Option<PathBuf>) -> bool {
    // Must be a symlink or junction (not a plain directory)
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let is_reparse_or_symlink;

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        // FILE_ATTRIBUTE_REPARSE_POINT (0x400) covers both symlinks and junction points
        is_reparse_or_symlink = meta.file_attributes() & 0x400 != 0;
    }

    #[cfg(unix)]
    {
        is_reparse_or_symlink = meta.file_type().is_symlink();
    }

    if !is_reparse_or_symlink {
        return false;
    }

    // Strategy 1: resolve and compare canonical paths (works when global-agent dir exists)
    if let Some(ref ga_canonical) = global_agent_canonical {
        if let Ok(target) = fs::canonicalize(path) {
            if let Some(parent) = target.parent() {
                if parent == ga_canonical {
                    return true;
                }
            }
        }
    }

    // Strategy 2: read the junction/symlink target string and check for ".agents/skills" pattern
    // This handles the case where the global-agent dir itself doesn't exist yet.
    #[cfg(windows)]
    {
        if let Ok(target) = junction::get_target(path) {
            let target_str = target.to_string_lossy().to_lowercase();
            if target_str.contains(".agents") && target_str.contains("skills") {
                return true;
            }
        }
    }

    #[cfg(unix)]
    {
        if let Ok(target) = std::fs::read_link(path) {
            let target_str = target.to_string_lossy().to_lowercase();
            if target_str.contains(".agents") && target_str.contains("skills") {
                return true;
            }
        }
    }

    false
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
    fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))
}

/// Enable a skill for a specific tool by creating a junction/symlink
#[tauri::command]
fn enable_skill_for_tool(skill_path: String, tool_id: String) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法找到用户主目录".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("未知工具: {}", tool_id))?;

    let target_base = resolve_path(&home, def.skill_path);

    // Ensure the target base directory exists
    fs::create_dir_all(&target_base).map_err(|e| format!("无法创建目标目录: {}", e))?;

    let source = PathBuf::from(&skill_path);
    let skill_name = source
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| "无效的 skill 路径".to_string())?;

    let dest = target_base.join(&skill_name);

    // Idempotent: if destination already exists, treat as success
    if dest.exists() || dest.is_symlink() {
        return Ok(dest.to_string_lossy().to_string());
    }

    // Create junction/symlink
    create_symlink_dir(&source, &dest).map_err(|e| format!("创建链接失败: {}", e))?;

    Ok(dest.to_string_lossy().to_string())
}

/// Disable a skill for a specific tool by removing it
#[tauri::command]
fn disable_skill_for_tool(skill_name: String, tool_id: String) -> Result<(), String> {
    let home = dirs::home_dir().ok_or_else(|| "无法找到用户主目录".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("未知工具: {}", tool_id))?;

    let target = resolve_path(&home, def.skill_path).join(&skill_name);

    // If it doesn't exist, treat as already disabled (idempotent)
    if !target.exists() && !target.is_symlink() {
        return Ok(());
    }

    fs::remove_dir_all(&target).map_err(|e| format!("删除 skill 失败: {}", e))?;

    Ok(())
}

/// Create a new skill in a specific tool (or global-agent)
#[tauri::command]
fn create_skill(
    tool_id: String,
    skill_name: String,
    description: String,
) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法找到用户主目录".to_string())?;
    let defs = get_tool_definitions();

    let def = defs
        .iter()
        .find(|d| d.id == tool_id.as_str())
        .ok_or_else(|| format!("未知工具: {}", tool_id))?;

    let base = resolve_path(&home, def.skill_path);
    fs::create_dir_all(&base).map_err(|e| format!("无法创建目录: {}", e))?;

    let skill_dir = base.join(&skill_name);

    if skill_dir.exists() {
        return Err("同名 Skill 已存在".to_string());
    }

    fs::create_dir_all(&skill_dir).map_err(|e| format!("无法创建 skill 目录: {}", e))?;

    let content = format!(
        "---\nname: {}\ndescription: {}\n---\n\n# {}\n\n{}\n",
        skill_name, description, skill_name, description
    );

    let skill_md = skill_dir.join("SKILL.md");
    fs::write(&skill_md, content).map_err(|e| format!("无法写入 SKILL.md: {}", e))?;

    Ok(skill_dir.to_string_lossy().to_string())
}

/// Delete a skill entirely from its source
#[tauri::command]
fn delete_skill(skill_path: String) -> Result<(), String> {
    let path = PathBuf::from(&skill_path);
    if !path.exists() {
        return Err("Skill 路径不存在".to_string());
    }

    fs::remove_dir_all(&path).map_err(|e| format!("删除 skill 失败: {}", e))?;

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
            .map_err(|e| format!("打开文件资源管理器失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开 Finder 失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件管理器失败: {}", e))?;
    }
    Ok(())
}

/// Create a directory symlink, handling cross-platform differences.
/// On Windows, junction points are used as they don't require elevated privileges.
fn create_symlink_dir(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    // Convert src to canonical absolute path for better symlink stability
    let src_absolute = fs::canonicalize(src)?;

    #[cfg(unix)]
    {
        unix_fs::symlink(&src_absolute, dst)?;
    }

    #[cfg(windows)]
    {
        // On Windows, symlink_dir requires either administrator privileges or
        // Developer Mode to be enabled. Junction points work without any special
        // privileges and behave equivalently for directory linking purposes.
        junction::create(&src_absolute, dst).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("无法创建目录链接: {}", e),
            )
        })?;
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
