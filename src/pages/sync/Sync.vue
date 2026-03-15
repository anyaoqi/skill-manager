<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSkillStore } from '../../stores/skillStore'
import { useToolStore } from '../../stores/toolStore'
import {
  RefreshCw,
  Check,
  X,
  AlertTriangle,
} from 'lucide-vue-next'

const skillStore = useSkillStore()
const toolStore = useToolStore()

const syncing = ref<string | null>(null)

const skillsWithIssues = computed(() => {
  return skillStore.skills.filter(skill => {
    return skill.tools.some(t => !t.synced || t.conflict)
  })
})

const getToolName = (toolId: string) => {
  const tool = toolStore.getToolById(toolId)
  return tool?.name || toolId
}

const handleSync = async (skillId: string) => {
  syncing.value = skillId
  const skill = skillStore.getSkillById(skillId)
  if (skill) {
    for (const tool of skill.tools) {
      if (tool.installed && !tool.synced) {
        await skillStore.syncSkillToTool(skillId, tool.toolId)
        await new Promise(resolve => setTimeout(resolve, 500))
      }
    }
  }
  syncing.value = null
}

const handleSyncAll = async () => {
  for (const skill of skillsWithIssues.value) {
    await handleSync(skill.id)
  }
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">Sync Center</h1>
        <p class="text-cyber-muted mt-1">Synchronize skills across all your tools</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        :disabled="!!syncing"
        @click="handleSyncAll"
      >
        <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': syncing }" />
        <span>Sync All</span>
      </button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="cyber-card text-center">
        <p class="text-3xl font-bold text-cyber-primary">{{ skillStore.totalSkills }}</p>
        <p class="text-cyber-muted text-sm">Total Skills</p>
      </div>
      <div class="cyber-card text-center">
        <p class="text-3xl font-bold text-cyber-success">
          {{ skillStore.skills.reduce((sum, s) => sum + s.tools.filter(t => t.synced).length, 0) }}
        </p>
        <p class="text-cyber-muted text-sm">Synced Instances</p>
      </div>
      <div class="cyber-card text-center">
        <p class="text-3xl font-bold" :class="skillStore.conflictCount > 0 ? 'text-cyber-error' : 'text-cyber-muted'">
          {{ skillStore.conflictCount }}
        </p>
        <p class="text-cyber-muted text-sm">Conflicts</p>
      </div>
    </div>

    <div class="space-y-4">
      <h2 class="text-lg font-semibold text-cyber-text">Skills to Sync</h2>
      
      <div v-if="skillsWithIssues.length === 0" class="cyber-card text-center py-8">
        <Check class="w-12 h-12 text-cyber-success mx-auto mb-4" />
        <h3 class="text-lg font-medium text-cyber-text mb-2">All Synced!</h3>
        <p class="text-cyber-muted">All your skills are up to date across all tools.</p>
      </div>

      <div
        v-for="skill in skillsWithIssues"
        :key="skill.id"
        class="cyber-card"
      >
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center">
              <span class="text-cyber-primary font-medium">{{ skill.name.charAt(0).toUpperCase() }}</span>
            </div>
            <div>
              <h3 class="font-semibold text-cyber-text">{{ skill.name }}</h3>
              <p class="text-xs text-cyber-muted">v{{ skill.version }}</p>
            </div>
          </div>
          
          <button 
            class="cyber-btn-primary flex items-center gap-2"
            :disabled="syncing === skill.id"
            @click="handleSync(skill.id)"
          >
            <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': syncing === skill.id }" />
            <span>Sync Now</span>
          </button>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
          <div 
            v-for="toolStatus in skill.tools" 
            :key="toolStatus.toolId"
            class="flex items-center gap-2 p-2 rounded-lg bg-cyber-bg"
          >
            <div 
              class="w-6 h-6 rounded flex items-center justify-center"
              :class="toolStatus.conflict ? 'bg-cyber-warning/20 text-cyber-warning' :
                     toolStatus.synced ? 'bg-cyber-success/20 text-cyber-success' :
                     toolStatus.installed ? 'bg-cyber-muted/20 text-cyber-muted' :
                     'bg-cyber-border text-cyber-muted'"
            >
              <Check v-if="toolStatus.synced" class="w-3 h-3" />
              <AlertTriangle v-else-if="toolStatus.conflict" class="w-3 h-3" />
              <X v-else-if="!toolStatus.installed" class="w-3 h-3" />
              <RefreshCw v-else class="w-3 h-3" />
            </div>
            <span class="text-sm text-cyber-text">{{ getToolName(toolStatus.toolId) }}</span>
          </div>
        </div>

        <div v-if="skill.tools.some(t => t.conflict)" class="mt-3 pt-3 border-t border-cyber-border">
          <p class="text-sm text-cyber-warning flex items-center gap-2">
            <AlertTriangle class="w-4 h-4" />
            This skill has conflicts that need to be resolved
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
