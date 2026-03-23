import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { DiscoveredSkill, ToolConfig, ScanResult, UniqueSkill } from '../types'

export const useSkillStore = defineStore('skills', () => {
  const skills = ref<DiscoveredSkill[]>([])
  const tools = ref<ToolConfig[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const searchQuery = ref('')
  const filterSource = ref<string | null>(null)

  // -------------------------------------------------------
  // Computed
  // -------------------------------------------------------

  /** Deduplicated skills, grouped by name */
  const uniqueSkills = computed<UniqueSkill[]>(() => {
    const map = new Map<string, UniqueSkill>()

    for (const skill of skills.value) {
      const existing = map.get(skill.name)
      if (existing) {
        existing.instances.push(skill)
        if (!existing.sources.includes(skill.source)) {
          existing.sources.push(skill.source)
        }
        // Merge enabled tools
        for (const t of skill.enabled_tools) {
          if (!existing.enabledTools.includes(t)) {
            existing.enabledTools.push(t)
          }
        }
        // Also add the source itself as enabled (except global-agent which is a source, not a tool)
        if (skill.source !== 'global-agent' && !existing.enabledTools.includes(skill.source)) {
          existing.enabledTools.push(skill.source)
        }
      } else {
        const enabledTools = [...skill.enabled_tools]
        if (skill.source !== 'global-agent' && !enabledTools.includes(skill.source)) {
          enabledTools.push(skill.source)
        }
        map.set(skill.name, {
          name: skill.name,
          description: skill.description || '',
          instances: [skill],
          sources: [skill.source],
          enabledTools,
          primaryPath: skill.path,
          primarySource: skill.source,
          primarySourceLabel: skill.source_label,
          files: skill.files,
        })
      }
    }

    // Prefer global-agent as primary source if available
    for (const unique of map.values()) {
      const globalInstance = unique.instances.find(i => i.source === 'global-agent')
      if (globalInstance) {
        unique.primaryPath = globalInstance.path
        unique.primarySource = globalInstance.source
        unique.primarySourceLabel = globalInstance.source_label
        unique.files = globalInstance.files
        unique.description = globalInstance.description || unique.description

        // For global-agent skills, auto-include all tools that support global agent
        for (const tool of tools.value) {
          if (tool.supports_global_agent && !unique.enabledTools.includes(tool.id)) {
            unique.enabledTools.push(tool.id)
          }
        }
      }
    }

    return Array.from(map.values())
  })

  /** Filtered skills based on search and source filter */
  const filteredSkills = computed(() => {
    return uniqueSkills.value.filter(skill => {
      const matchesSearch = !searchQuery.value ||
        skill.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        skill.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      const matchesSource = !filterSource.value ||
        skill.sources.includes(filterSource.value)
      return matchesSearch && matchesSource
    })
  })

  /** Global-agent skills */
  const globalAgentSkills = computed(() =>
    uniqueSkills.value.filter(s => s.sources.includes('global-agent'))
  )

  /** Tool-specific-only skills (NOT in global agent) */
  const toolSpecificSkills = computed(() =>
    uniqueSkills.value.filter(s => !s.sources.includes('global-agent'))
  )

  /** Available tools (excluding global-agent) */
  const availableTools = computed(() =>
    tools.value.filter(t => t.id !== 'global-agent')
  )

  /** Global agent tool config */
  const globalAgentTool = computed(() =>
    tools.value.find(t => t.id === 'global-agent')
  )

  /** All source options for filtering */
  const sourceOptions = computed(() => {
    const sources = new Set<string>()
    skills.value.forEach(s => sources.add(s.source))
    return Array.from(sources)
  })

  const totalSkillCount = computed(() => uniqueSkills.value.length)
  const globalSkillCount = computed(() => globalAgentSkills.value.length)
  const toolSpecificCount = computed(() => toolSpecificSkills.value.length)

  // -------------------------------------------------------
  // Actions
  // -------------------------------------------------------

  const scanAllSkills = async () => {
    loading.value = true
    error.value = null
    try {
      const result = await invoke<ScanResult>('scan_all_skills')
      skills.value = result.skills
      tools.value = result.tools
    } catch (e: any) {
      error.value = e?.toString() || 'Failed to scan skills'
      console.error('Scan error:', e)
    } finally {
      loading.value = false
    }
  }

  const enableSkillForTool = async (skillPath: string, toolId: string) => {
    try {
      await invoke('enable_skill_for_tool', { skillPath, toolId })
      await scanAllSkills()
    } catch (e: any) {
      throw new Error(e?.toString() || 'Failed to enable skill')
    }
  }

  const disableSkillForTool = async (skillName: string, toolId: string) => {
    try {
      await invoke('disable_skill_for_tool', { skillName, toolId })
      await scanAllSkills()
    } catch (e: any) {
      throw new Error(e?.toString() || 'Failed to disable skill')
    }
  }

  const createSkill = async (toolId: string, skillName: string, description: string) => {
    try {
      await invoke('create_skill', { toolId, skillName, description })
      await scanAllSkills()
    } catch (e: any) {
      throw new Error(e?.toString() || 'Failed to create skill')
    }
  }

  const deleteSkill = async (skillPath: string) => {
    try {
      await invoke('delete_skill', { skillPath })
      await scanAllSkills()
    } catch (e: any) {
      throw new Error(e?.toString() || 'Failed to delete skill')
    }
  }

  const openInExplorer = async (path: string) => {
    try {
      await invoke('open_in_explorer', { path })
    } catch (e: any) {
      console.error('Failed to open explorer:', e)
    }
  }

  const readSkillFile = async (filePath: string): Promise<string> => {
    try {
      return await invoke<string>('read_skill_file', { filePath })
    } catch (e: any) {
      throw new Error(e?.toString() || 'Failed to read file')
    }
  }

  const getUniqueSkillByName = (name: string): UniqueSkill | undefined => {
    return uniqueSkills.value.find(s => s.name === name)
  }

  /** Check if a specific tool has a specific skill */
  const isSkillEnabledForTool = (skillName: string, toolId: string): boolean => {
    return skills.value.some(s => s.name === skillName && s.source === toolId)
  }

  return {
    // State
    skills,
    tools,
    loading,
    error,
    searchQuery,
    filterSource,
    // Computed
    uniqueSkills,
    filteredSkills,
    globalAgentSkills,
    toolSpecificSkills,
    availableTools,
    globalAgentTool,
    sourceOptions,
    totalSkillCount,
    globalSkillCount,
    toolSpecificCount,
    // Actions
    scanAllSkills,
    enableSkillForTool,
    disableSkillForTool,
    createSkill,
    deleteSkill,
    openInExplorer,
    readSkillFile,
    getUniqueSkillByName,
    isSkillEnabledForTool,
  }
})
