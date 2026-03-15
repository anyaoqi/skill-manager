import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Skill } from '../types'

export const useSkillStore = defineStore('skills', () => {
  const skills = ref<Skill[]>([
    {
      id: '1',
      name: 'vue-expert',
      description: 'Vue.js coding assistant with composition API expertise',
      version: '2.1.0',
      tags: ['vue', 'frontend', 'composition-api'],
      files: [
        { name: 'SKILL.md', type: 'file', path: '/SKILL.md' },
        { name: 'prompts', type: 'directory', path: '/prompts' },
      ],
      tools: [
        { toolId: 'cursor', installed: true, synced: true, lastSynced: '2 min ago' },
        { toolId: 'vscode', installed: true, synced: true, lastSynced: '5 min ago' },
        { toolId: 'openclaw', installed: true, synced: true, lastSynced: '1 min ago' },
        { toolId: 'trae', installed: false, synced: false },
      ],
      createdAt: '2024-01-15',
      updatedAt: '2024-03-10',
    },
    {
      id: '2',
      name: 'react-master',
      description: 'React development expert with hooks and TypeScript',
      version: '1.8.2',
      tags: ['react', 'frontend', 'typescript'],
      files: [
        { name: 'SKILL.md', type: 'file', path: '/SKILL.md' },
      ],
      tools: [
        { toolId: 'cursor', installed: true, synced: true, lastSynced: '10 min ago' },
        { toolId: 'vscode', installed: true, synced: true, lastSynced: '10 min ago' },
        { toolId: 'openclaw', installed: true, synced: false, conflict: true },
        { toolId: 'trae', installed: false, synced: false },
      ],
      createdAt: '2024-02-01',
      updatedAt: '2024-03-08',
    },
    {
      id: '3',
      name: 'debug-agent',
      description: 'AI-powered debugging assistant for all languages',
      version: '1.2.0',
      tags: ['debug', 'general'],
      files: [
        { name: 'SKILL.md', type: 'file', path: '/SKILL.md' },
        { name: 'debug-prompts', type: 'directory', path: '/debug-prompts' },
      ],
      tools: [
        { toolId: 'cursor', installed: true, synced: true, lastSynced: '1 hour ago' },
        { toolId: 'vscode', installed: false, synced: false },
        { toolId: 'openclaw', installed: true, synced: true, lastSynced: '30 min ago' },
        { toolId: 'trae', installed: true, synced: true, lastSynced: '2 hours ago' },
      ],
      createdAt: '2024-01-20',
      updatedAt: '2024-03-05',
    },
    {
      id: '4',
      name: 'frontend-ui-ux',
      description: 'Designer-turned-developer for stunning UI/UX without mockups',
      version: '1.0.2',
      tags: ['ui', 'ux', 'design', 'frontend'],
      files: [
        { name: 'SKILL.md', type: 'file', path: '/SKILL.md' },
      ],
      tools: [
        { toolId: 'cursor', installed: false, synced: false },
        { toolId: 'vscode', installed: false, synced: false },
        { toolId: 'openclaw', installed: true, synced: true, lastSynced: '5 min ago' },
        { toolId: 'trae', installed: false, synced: false },
      ],
      createdAt: '2024-03-01',
      updatedAt: '2024-03-15',
    },
  ])

  const selectedSkill = ref<Skill | null>(null)
  const searchQuery = ref('')
  const filterTag = ref<string | null>(null)

  const filteredSkills = computed(() => {
    return skills.value.filter(skill => {
      const matchesSearch = skill.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        skill.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      const matchesTag = !filterTag.value || skill.tags.includes(filterTag.value)
      return matchesSearch && matchesTag
    })
  })

  const allTags = computed(() => {
    const tags = new Set<string>()
    skills.value.forEach(skill => skill.tags.forEach(tag => tags.add(tag)))
    return Array.from(tags)
  })

  const totalSkills = computed(() => skills.value.length)

  const syncedToolsCount = computed(() => {
    const tools = new Set<string>()
    skills.value.forEach(skill => {
      skill.tools.forEach(tool => {
        if (tool.synced) tools.add(tool.toolId)
      })
    })
    return tools.size
  })

  const conflictCount = computed(() => {
    return skills.value.reduce((count, skill) => {
      return count + skill.tools.filter(t => t.conflict).length
    }, 0)
  })

  const getSkillById = (id: string) => {
    return skills.value.find(skill => skill.id === id)
  }

  const addSkill = (skill: Omit<Skill, 'id' | 'createdAt' | 'updatedAt'>) => {
    const newSkill: Skill = {
      ...skill,
      id: Date.now().toString(),
      createdAt: new Date().toISOString().split('T')[0],
      updatedAt: new Date().toISOString().split('T')[0],
    }
    skills.value.push(newSkill)
  }

  const updateSkill = (id: string, updates: Partial<Skill>) => {
    const index = skills.value.findIndex(s => s.id === id)
    if (index !== -1) {
      skills.value[index] = {
        ...skills.value[index],
        ...updates,
        updatedAt: new Date().toISOString().split('T')[0],
      }
    }
  }

  const deleteSkill = (id: string) => {
    const index = skills.value.findIndex(s => s.id === id)
    if (index !== -1) {
      skills.value.splice(index, 1)
    }
  }

  const syncSkillToTool = async (skillId: string, toolId: string) => {
    const skill = skills.value.find(s => s.id === skillId)
    if (skill) {
      const toolStatus = skill.tools.find(t => t.toolId === toolId)
      if (toolStatus) {
        toolStatus.synced = true
        toolStatus.lastSynced = 'just now'
        toolStatus.conflict = false
      }
    }
  }

  return {
    skills,
    selectedSkill,
    searchQuery,
    filterTag,
    filteredSkills,
    allTags,
    totalSkills,
    syncedToolsCount,
    conflictCount,
    getSkillById,
    addSkill,
    updateSkill,
    deleteSkill,
    syncSkillToTool,
  }
})
