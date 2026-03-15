<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import { useToolStore } from '../../stores/toolStore'
import {
  Boxes,
  Plug,
  Clock,
  AlertTriangle,
  ArrowRight,
  Sparkles,
  FolderSync,
} from 'lucide-vue-next'

const router = useRouter()
const skillStore = useSkillStore()
const toolStore = useToolStore()

const stats = computed(() => [
  {
    label: 'Total Skills',
    value: skillStore.totalSkills,
    icon: Boxes,
    color: 'primary',
    bg: 'from-cyber-primary/20 to-cyber-primary/5',
  },
  {
    label: 'Connected Tools',
    value: toolStore.connectedTools.length,
    icon: Plug,
    color: 'success',
    bg: 'from-cyber-success/20 to-cyber-success/5',
  },
  {
    label: 'Last Sync',
    value: '2 min ago',
    icon: Clock,
    color: 'secondary',
    bg: 'from-cyber-secondary/20 to-cyber-secondary/5',
  },
  {
    label: 'Conflicts',
    value: skillStore.conflictCount,
    icon: AlertTriangle,
    color: skillStore.conflictCount > 0 ? 'error' : 'muted',
    bg: skillStore.conflictCount > 0 
      ? 'from-cyber-error/20 to-cyber-error/5' 
      : 'from-cyber-border to-cyber-border/50',
  },
])

const recentSkills = computed(() => skillStore.filteredSkills.slice(0, 4))

const getToolIcon = (toolId: string) => {
  const icons: Record<string, string> = {
    cursor: '⬡',
    vscode: '⬡',
    openclaw: '🦞',
    trae: '⚡',
    'claude-code': '👤',
  }
  return icons[toolId] || '○'
}

const getSyncStatus = (tools: { toolId: string; synced: boolean; conflict?: boolean }[]) => {
  if (tools.some(t => t.conflict)) return 'conflict'
  if (tools.every(t => t.synced)) return 'synced'
  if (tools.some(t => t.synced)) return 'partial'
  return 'not-synced'
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">Dashboard</h1>
        <p class="text-cyber-muted mt-1">Manage AI Skills Across Tools</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="router.push('/skills')"
      >
        <Sparkles class="w-4 h-4" />
        <span>Add Skill</span>
      </button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <div
        v-for="stat in stats"
        :key="stat.label"
        class="cyber-card-hover group"
      >
        <div class="flex items-start justify-between">
          <div>
            <p class="text-cyber-muted text-sm">{{ stat.label }}</p>
            <p 
              class="text-3xl font-bold mt-2"
              :class="stat.color === 'primary' ? 'text-cyber-primary' : 
                     stat.color === 'success' ? 'text-cyber-success' :
                     stat.color === 'secondary' ? 'text-cyber-secondary' :
                     stat.color === 'error' ? 'text-cyber-error' :
                     'text-cyber-muted'"
            >
              {{ stat.value }}
            </p>
          </div>
          <div 
            class="w-12 h-12 rounded-xl bg-gradient-to-br flex items-center justify-center transition-transform group-hover:scale-110"
            :class="stat.bg"
          >
            <component 
              :is="stat.icon" 
              class="w-6 h-6"
              :class="stat.color === 'primary' ? 'text-cyber-primary' : 
                     stat.color === 'success' ? 'text-cyber-success' :
                     stat.color === 'secondary' ? 'text-cyber-secondary' :
                     stat.color === 'error' ? 'text-cyber-error' :
                     'text-cyber-muted'"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold text-cyber-text">Recent Skills</h2>
          <button 
            class="text-cyber-primary text-sm flex items-center gap-1 hover:gap-2 transition-all"
            @click="router.push('/skills')"
          >
            View All <ArrowRight class="w-4 h-4" />
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div
            v-for="skill in recentSkills"
            :key="skill.id"
            class="cyber-card-hover cursor-pointer group"
            @click="router.push(`/skills/${skill.id}`)"
          >
            <div class="flex items-start justify-between mb-3">
              <div class="flex items-center gap-2">
                <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center">
                  <Sparkles class="w-5 h-5 text-cyber-primary" />
                </div>
                <div>
                  <h3 class="font-semibold text-cyber-text group-hover:text-cyber-primary transition-colors">
                    {{ skill.name }}
                  </h3>
                  <p class="text-xs text-cyber-muted">v{{ skill.version }}</p>
                </div>
              </div>
              <span 
                class="px-2 py-1 rounded-full text-xs font-medium"
                :class="getSyncStatus(skill.tools) === 'synced' ? 'bg-cyber-success/20 text-cyber-success' :
                       getSyncStatus(skill.tools) === 'conflict' ? 'bg-cyber-error/20 text-cyber-error' :
                       getSyncStatus(skill.tools) === 'partial' ? 'bg-cyber-warning/20 text-cyber-warning' :
                       'bg-cyber-muted/20 text-cyber-muted'"
              >
                {{ getSyncStatus(skill.tools) === 'synced' ? 'synced' :
                  getSyncStatus(skill.tools) === 'conflict' ? 'conflict' :
                  getSyncStatus(skill.tools) === 'partial' ? 'partial' :
                  'not synced' }}
              </span>
            </div>
            
            <p class="text-sm text-cyber-muted line-clamp-2 mb-3">
              {{ skill.description }}
            </p>

            <div class="flex flex-wrap gap-1 mb-3">
              <span 
                v-for="tag in skill.tags.slice(0, 3)" 
                :key="tag"
                class="px-2 py-0.5 rounded bg-cyber-border text-xs text-cyber-muted"
              >
                {{ tag }}
              </span>
            </div>

            <div class="flex items-center gap-2 pt-3 border-t border-cyber-border">
              <span 
                v-for="tool in skill.tools.filter(t => t.installed).slice(0, 4)" 
                :key="tool.toolId"
                class="w-6 h-6 rounded flex items-center justify-center text-xs"
                :class="tool.conflict ? 'bg-cyber-error/20' : 'bg-cyber-card'"
                :title="tool.toolId"
              >
                {{ getToolIcon(tool.toolId) }}
              </span>
              <span class="text-xs text-cyber-muted ml-auto">
                {{ skill.tools.filter(t => t.installed).length }} tools
              </span>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold text-cyber-text">Connected Tools</h2>
          <button 
            class="text-cyber-primary text-sm flex items-center gap-1 hover:gap-2 transition-all"
            @click="router.push('/tools')"
          >
            Manage <ArrowRight class="w-4 h-4" />
          </button>
        </div>

        <div class="space-y-3">
          <div
            v-for="tool in toolStore.connectedTools"
            :key="tool.id"
            class="cyber-card flex items-center gap-3"
          >
            <div class="w-10 h-10 rounded-lg bg-cyber-card flex items-center justify-center text-lg">
              {{ getToolIcon(tool.id) }}
            </div>
            <div class="flex-1 min-w-0">
              <h3 class="font-medium text-cyber-text truncate">{{ tool.name }}</h3>
              <p class="text-xs text-cyber-muted">{{ tool.skillsCount }} skills</p>
            </div>
            <div class="w-2 h-2 rounded-full bg-cyber-success animate-pulse"></div>
          </div>

          <div
            v-for="tool in toolStore.notFoundTools.slice(0, 2)"
            :key="tool.id"
            class="cyber-card flex items-center gap-3 opacity-60"
          >
            <div class="w-10 h-10 rounded-lg bg-cyber-card flex items-center justify-center text-lg">
              {{ getToolIcon(tool.id) }}
            </div>
            <div class="flex-1 min-w-0">
              <h3 class="font-medium text-cyber-text truncate">{{ tool.name }}</h3>
              <p class="text-xs text-cyber-muted">Not installed</p>
            </div>
            <div class="w-2 h-2 rounded-full bg-cyber-muted"></div>
          </div>
        </div>

        <button 
          class="cyber-card w-full flex items-center justify-center gap-2 text-cyber-muted hover:text-cyber-primary transition-colors"
          @click="router.push('/sync')"
        >
          <FolderSync class="w-4 h-4" />
          <span>Open Sync Center</span>
        </button>
      </div>
    </div>
  </div>
</template>
