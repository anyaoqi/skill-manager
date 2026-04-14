// Type definitions for SkillManager

export interface SkillFileEntry {
  name: string
  path: string
  is_dir: boolean
}

export interface DiscoveredSkill {
  id: string
  name: string
  description: string
  path: string
  source: string
  source_label: string
  files: SkillFileEntry[]
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

export interface UniqueSkill {
  name: string
  description: string
  instances: DiscoveredSkill[]
  sources: string[]
  enabledTools: string[]
  primaryPath: string
  primarySource: string
  primarySourceLabel: string
  files: SkillFileEntry[]
}

export interface AppSettings {
  theme: string
  autoScan: boolean
  notifications: boolean
}