// ============================================================
// Core Types
// ============================================================

export interface SkillFileEntry {
  name: string
  path: string
  is_dir: boolean
}

/** A skill discovered from the file system */
export interface DiscoveredSkill {
  id: string
  name: string
  description: string
  path: string
  /** "global-agent" or a tool id like "opencode", "cursor" */
  source: string
  source_label: string
  files: SkillFileEntry[]
  /** Tool IDs that currently have this skill */
  enabled_tools: string[]
}

export interface ToolConfig {
  id: string
  name: string
  skill_path: string
  resolved_path: string
  exists: boolean
  skill_count: number
  supports_global_agent: boolean
}

export interface ScanResult {
  skills: DiscoveredSkill[]
  tools: ToolConfig[]
}

export interface AppSettings {
  theme: 'dark' | 'light' | 'system'
  autoScan: boolean
  notifications: boolean
}

/** Unique skill — deduplicated by name across all sources */
export interface UniqueSkill {
  name: string
  description: string
  /** All source-specific instances */
  instances: DiscoveredSkill[]
  /** Which sources have this skill: 'global-agent', 'opencode', etc. */
  sources: string[]
  /** Which tool IDs have this skill (union of all enabled_tools + sources excluding global-agent) */
  enabledTools: string[]
  /** Primary path (from the first instance found, preferring global-agent) */
  primaryPath: string
  /** Primary source */
  primarySource: string
  primarySourceLabel: string
  files: SkillFileEntry[]
}
