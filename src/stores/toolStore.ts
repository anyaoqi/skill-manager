import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Tool } from '../types'

export const useToolStore = defineStore('tools', () => {
  const tools = ref<Tool[]>([
    {
      id: 'cursor',
      name: 'Cursor',
      icon: 'cursor',
      path: '~/.cursor/skills',
      status: 'connected',
      skillsCount: 12,
    },
    {
      id: 'vscode',
      name: 'VS Code',
      icon: 'vscode',
      path: '~/.vscode/extensions',
      status: 'connected',
      skillsCount: 8,
    },
    {
      id: 'openclaw',
      name: 'OpenClaw',
      icon: 'openclaw',
      path: '~/.openclaw/skills',
      status: 'connected',
      skillsCount: 4,
    },
    {
      id: 'trae',
      name: 'Trae',
      icon: 'trae',
      path: '~/.trae/skills',
      status: 'not-found',
      skillsCount: 0,
    },
    {
      id: 'windsurf',
      name: 'Windsurf',
      icon: 'windsurf',
      path: '~/.windsurf/skills',
      status: 'not-found',
      skillsCount: 0,
    },
    {
      id: 'claude-code',
      name: 'Claude Code',
      icon: 'claude',
      path: '~/.claude/skills',
      status: 'connected',
      skillsCount: 6,
    },
  ])

  const selectedTool = ref<Tool | null>(null)

  const connectedTools = computed(() => 
    tools.value.filter(t => t.status === 'connected')
  )

  const disconnectedTools = computed(() => 
    tools.value.filter(t => t.status === 'disconnected')
  )

  const notFoundTools = computed(() => 
    tools.value.filter(t => t.status === 'not-found')
  )

  const getToolById = (id: string) => {
    return tools.value.find(tool => tool.id === id)
  }

  const updateToolPath = (id: string, path: string) => {
    const tool = tools.value.find(t => t.id === id)
    if (tool) {
      tool.path = path
      tool.customPath = path
    }
  }

  const setToolStatus = (id: string, status: 'connected' | 'disconnected' | 'not-found') => {
    const tool = tools.value.find(t => t.id === id)
    if (tool) {
      tool.status = status
    }
  }

  const refreshTools = async () => {
    // In a real app, this would scan the filesystem
    // For now, we'll just simulate a refresh
    console.log('Refreshing tools...')
  }

  return {
    tools,
    selectedTool,
    connectedTools,
    disconnectedTools,
    notFoundTools,
    getToolById,
    updateToolPath,
    setToolStatus,
    refreshTools,
  }
})
