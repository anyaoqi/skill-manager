<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import { useToolStore } from '../../stores/toolStore'
import {
  ArrowLeft,
  Edit,
  Trash2,
  Folder,
  FileText,
  Check,
  RefreshCw,
  AlertTriangle,
} from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const skillStore = useSkillStore()
const toolStore = useToolStore()

const skill = computed(() => skillStore.getSkillById(route.params.id as string))

const getToolName = (toolId: string) => {
  const tool = toolStore.getToolById(toolId)
  return tool?.name || toolId
}

const handleSync = async (toolId: string) => {
  if (skill.value) {
    await skillStore.syncSkillToTool(skill.value.id, toolId)
  }
}
</script>

<template>
  <div v-if="skill" class="space-y-6">
    <div class="flex items-center gap-4">
      <button 
        class="p-2 rounded-lg hover:bg-cyber-card text-cyber-muted hover:text-cyber-text transition-colors"
        @click="router.back()"
      >
        <ArrowLeft class="w-5 h-5" />
      </button>
      <div class="flex-1">
        <h1 class="text-2xl font-bold text-cyber-text">{{ skill.name }}</h1>
        <p class="text-cyber-muted mt-1">v{{ skill.version }}</p>
      </div>
      <button class="cyber-btn-secondary flex items-center gap-2">
        <Edit class="w-4 h-4" />
        <span>Edit</span>
      </button>
      <button 
        class="cyber-btn bg-cyber-error/10 text-cyber-error border border-cyber-error/30 hover:bg-cyber-error/20 flex items-center gap-2"
        @click="skillStore.deleteSkill(skill.id); router.back()"
      >
        <Trash2 class="w-4 h-4" />
        <span>Delete</span>
      </button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 space-y-6">
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">Description</h2>
          <p class="text-cyber-muted">{{ skill.description }}</p>
          
          <div class="flex flex-wrap gap-2 mt-4">
            <span 
              v-for="tag in skill.tags" 
              :key="tag"
              class="px-3 py-1 rounded-full bg-cyber-primary/10 text-cyber-primary text-sm"
            >
              {{ tag }}
            </span>
          </div>
        </div>

        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">Files</h2>
          <div class="space-y-2">
            <div 
              v-for="file in skill.files" 
              :key="file.path"
              class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg hover:bg-cyber-border/30 cursor-pointer transition-colors"
            >
              <component 
                :is="file.type === 'directory' ? Folder : FileText" 
                class="w-5 h-5 text-cyber-primary"
              />
              <span class="text-cyber-text">{{ file.name }}</span>
              <span class="text-cyber-muted text-sm ml-auto">{{ file.type }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">Tools Status</h2>
          <div class="space-y-3">
            <div 
              v-for="toolStatus in skill.tools" 
              :key="toolStatus.toolId"
              class="flex items-center justify-between p-3 rounded-lg bg-cyber-bg"
            >
              <div class="flex items-center gap-3">
                <div 
                  class="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-medium"
                  :class="toolStatus.installed ? 'bg-cyber-success/20 text-cyber-success' : 'bg-cyber-muted/20 text-cyber-muted'"
                >
                  {{ toolStatus.installed ? '✓' : '✗' }}
                </div>
                <div>
                  <p class="text-cyber-text font-medium">{{ getToolName(toolStatus.toolId) }}</p>
                  <p class="text-xs text-cyber-muted">
                    {{ toolStatus.lastSynced || 'Not synced' }}
                  </p>
                </div>
              </div>
              
              <div class="flex items-center gap-2">
                <span 
                  v-if="toolStatus.conflict"
                  class="px-2 py-1 rounded-full text-xs bg-cyber-warning/20 text-cyber-warning flex items-center gap-1"
                >
                  <AlertTriangle class="w-3 h-3" />
                  Conflict
                </span>
                <span 
                  v-else-if="toolStatus.synced"
                  class="px-2 py-1 rounded-full text-xs bg-cyber-success/20 text-cyber-success flex items-center gap-1"
                >
                  <Check class="w-3 h-3" />
                  Synced
                </span>
                <span 
                  v-else-if="toolStatus.installed"
                  class="px-2 py-1 rounded-full text-xs bg-cyber-muted/20 text-cyber-muted"
                >
                  Not synced
                </span>
                
                <button 
                  v-if="toolStatus.installed"
                  class="p-2 rounded-lg hover:bg-cyber-card text-cyber-muted hover:text-cyber-primary transition-colors"
                  @click="handleSync(toolStatus.toolId)"
                >
                  <RefreshCw class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">Quick Actions</h2>
          <div class="space-y-2">
            <button class="cyber-btn w-full justify-start gap-3 bg-cyber-bg hover:bg-cyber-border/30">
              <RefreshCw class="w-4 h-4 text-cyber-primary" />
              <span>Sync to All Tools</span>
            </button>
            <button class="cyber-btn w-full justify-start gap-3 bg-cyber-bg hover:bg-cyber-border/30">
              <FileText class="w-4 h-4 text-cyber-secondary" />
              <span>View Source</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div v-else class="flex items-center justify-center h-64">
    <div class="text-center">
      <AlertTriangle class="w-12 h-12 text-cyber-warning mx-auto mb-4" />
      <h2 class="text-xl font-semibold text-cyber-text mb-2">Skill Not Found</h2>
      <p class="text-cyber-muted mb-4">The skill you're looking for doesn't exist</p>
      <button 
        class="cyber-btn-primary"
        @click="router.push('/skills')"
      >
        Back to Skills
      </button>
    </div>
  </div>
</template>
