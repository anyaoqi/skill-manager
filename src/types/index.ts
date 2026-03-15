export interface Skill {
  id: string
  name: string
  description: string
  version: string
  tags: string[]
  files: SkillFile[]
  tools: ToolStatus[]
  createdAt: string
  updatedAt: string
}

export interface SkillFile {
  name: string
  type: 'file' | 'directory'
  path: string
}

export interface ToolStatus {
  toolId: string
  installed: boolean
  synced: boolean
  conflict?: boolean
  lastSynced?: string
}

export interface Tool {
  id: string
  name: string
  icon: string
  path: string
  status: 'connected' | 'disconnected' | 'not-found'
  skillsCount: number
  customPath?: string
}

export interface SyncRecord {
  id: string
  skillId: string
  toolId: string
  status: 'synced' | 'pending' | 'conflict' | 'failed'
  timestamp: string
  error?: string
}

export interface AppSettings {
  theme: 'dark' | 'light' | 'system'
  autoSync: boolean
  syncInterval: number
  notifications: boolean
  customToolPaths: Record<string, string>
}
