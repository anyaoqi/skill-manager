<script setup lang="ts">
import { ref } from 'vue'
import { useToolStore } from '../../stores/toolStore'
import {
  RefreshCw,
  FolderOpen,
  Check,
  X,
  AlertCircle,
  Settings,
} from 'lucide-vue-next'

const toolStore = useToolStore()

const editingPath = ref<string | null>(null)
const customPath = ref('')

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'connected': return { icon: Check, color: 'text-cyber-success', bg: 'bg-cyber-success/20' }
    case 'disconnected': return { icon: X, color: 'text-cyber-error', bg: 'bg-cyber-error/20' }
    default: return { icon: AlertCircle, color: 'text-cyber-warning', bg: 'bg-cyber-warning/20' }
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'connected': return 'Connected'
    case 'disconnected': return 'Disconnected'
    default: return 'Not Found'
  }
}

const startEditPath = (toolId: string, currentPath: string) => {
  editingPath.value = toolId
  customPath.value = currentPath
}

const savePath = (toolId: string) => {
  toolStore.updateToolPath(toolId, customPath.value)
  editingPath.value = null
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">Tools</h1>
        <p class="text-cyber-muted mt-1">Manage connected AI coding tools</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="toolStore.refreshTools()"
      >
        <RefreshCw class="w-4 h-4" />
        <span>Refresh</span>
      </button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div
        v-for="tool in toolStore.tools"
        :key="tool.id"
        class="cyber-card-hover"
      >
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center text-2xl">
              {{ tool.id === 'cursor' ? '⬡' : tool.id === 'vscode' ? '⬡' : tool.id === 'openclaw' ? '🦞' : tool.id === 'trae' ? '⚡' : '👤' }}
            </div>
            <div>
              <h3 class="font-semibold text-cyber-text">{{ tool.name }}</h3>
              <p class="text-xs text-cyber-muted">{{ tool.skillsCount }} skills</p>
            </div>
          </div>
          <div 
            class="px-2 py-1 rounded-full text-xs font-medium flex items-center gap-1"
            :class="getStatusIcon(tool.status).bg + ' ' + getStatusIcon(tool.status).color"
          >
            <component :is="getStatusIcon(tool.status).icon" class="w-3 h-3" />
            {{ getStatusText(tool.status) }}
          </div>
        </div>

        <div class="space-y-3">
          <div>
            <label class="block text-xs text-cyber-muted mb-1">Skills Path</label>
            <div v-if="editingPath === tool.id" class="flex gap-2">
              <input 
                v-model="customPath"
                type="text"
                class="cyber-input flex-1 text-sm"
                @keyup.enter="savePath(tool.id)"
              />
              <button 
                class="cyber-btn-primary px-3"
                @click="savePath(tool.id)"
              >
                Save
              </button>
            </div>
            <div v-else class="flex items-center gap-2">
              <p class="text-sm text-cyber-text flex-1 truncate">{{ tool.path }}</p>
              <button 
                class="p-1 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary"
                @click="startEditPath(tool.id, tool.path)"
              >
                <Settings class="w-4 h-4" />
              </button>
            </div>
          </div>

          <div v-if="tool.status === 'not-found'" class="pt-2 border-t border-cyber-border">
            <button 
              class="cyber-btn w-full text-sm justify-center"
            >
              <FolderOpen class="w-4 h-4" />
              <span>Set Custom Path</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="cyber-card">
      <h2 class="text-lg font-semibold text-cyber-text mb-4">Auto-Detection</h2>
      <p class="text-cyber-muted text-sm mb-4">
        SkillDock automatically scans common locations for AI coding tools. 
        You can manually specify custom paths above if your tools are installed elsewhere.
      </p>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-2 text-sm">
        <div class="p-2 rounded bg-cyber-bg">
          <p class="text-cyber-muted">Cursor</p>
          <p class="text-cyber-text text-xs">~/.cursor/skills</p>
        </div>
        <div class="p-2 rounded bg-cyber-bg">
          <p class="text-cyber-muted">VS Code</p>
          <p class="text-cyber-text text-xs">~/.vscode/extensions</p>
        </div>
        <div class="p-2 rounded bg-cyber-bg">
          <p class="text-cyber-muted">OpenClaw</p>
          <p class="text-cyber-text text-xs">~/.openclaw/skills</p>
        </div>
        <div class="p-2 rounded bg-cyber-bg">
          <p class="text-cyber-muted">Claude Code</p>
          <p class="text-cyber-text text-xs">~/.claude/skills</p>
        </div>
      </div>
    </div>
  </div>
</template>
