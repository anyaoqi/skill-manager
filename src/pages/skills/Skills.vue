<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import {
  Plus,
  Search,
  Grid,
  List,
  Sparkles,
  Edit,
  Trash2,
  Copy,
} from 'lucide-vue-next'

const router = useRouter()
const skillStore = useSkillStore()

const viewMode = ref<'grid' | 'list'>('grid')
const showAddModal = ref(false)

const newSkill = ref({
  name: '',
  description: '',
  version: '1.0.0',
  tags: '',
})

const handleAddSkill = () => {
  if (!newSkill.value.name) return
  
  skillStore.addSkill({
    name: newSkill.value.name,
    description: newSkill.value.description,
    version: newSkill.value.version,
    tags: newSkill.value.tags.split(',').map(t => t.trim()).filter(Boolean),
    files: [{ name: 'SKILL.md', type: 'file', path: '/SKILL.md' }],
    tools: [
      { toolId: 'cursor', installed: false, synced: false },
      { toolId: 'vscode', installed: false, synced: false },
      { toolId: 'openclaw', installed: false, synced: false },
      { toolId: 'trae', installed: false, synced: false },
    ],
  })
  
  newSkill.value = { name: '', description: '', version: '1.0.0', tags: '' }
  showAddModal.value = false
}

const getSyncStatus = (tools: { synced: boolean; conflict?: boolean }[]) => {
  if (tools.some(t => t.conflict)) return { label: 'Conflict', color: 'error' }
  if (tools.every(t => t.synced)) return { label: 'Synced', color: 'success' }
  if (tools.some(t => t.synced)) return { label: 'Partial', color: 'warning' }
  return { label: 'Not Synced', color: 'muted' }
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">Skills</h1>
        <p class="text-cyber-muted mt-1">Manage your AI coding skills</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="showAddModal = true"
      >
        <Plus class="w-4 h-4" />
        <span>Add Skill</span>
      </button>
    </div>

    <div class="flex items-center gap-4">
      <div class="flex-1 relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-cyber-muted" />
        <input 
          v-model="skillStore.searchQuery"
          type="text"
          placeholder="Search skills..."
          class="cyber-input w-full pl-10"
        />
      </div>
      
      <div class="flex items-center gap-2">
        <button
          v-for="tag in skillStore.allTags.slice(0, 5)"
          :key="tag"
          class="px-3 py-1.5 rounded-lg text-sm transition-all"
          :class="skillStore.filterTag === tag 
            ? 'bg-cyber-primary/20 text-cyber-primary border border-cyber-primary/30' 
            : 'bg-cyber-card text-cyber-muted border border-cyber-border hover:border-cyber-primary/30'"
          @click="skillStore.filterTag = skillStore.filterTag === tag ? null : tag"
        >
          {{ tag }}
        </button>
      </div>

      <div class="flex items-center gap-1 bg-cyber-card rounded-lg p-1 border border-cyber-border">
        <button 
          class="p-2 rounded transition-colors"
          :class="viewMode === 'grid' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted hover:text-cyber-text'"
          @click="viewMode = 'grid'"
        >
          <Grid class="w-4 h-4" />
        </button>
        <button 
          class="p-2 rounded transition-colors"
          :class="viewMode === 'list' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted hover:text-cyber-text'"
          @click="viewMode = 'list'"
        >
          <List class="w-4 h-4" />
        </button>
      </div>
    </div>

    <div 
      class="grid gap-4"
      :class="viewMode === 'grid' ? 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3' : 'grid-cols-1'"
    >
      <div
        v-for="skill in skillStore.filteredSkills"
        :key="skill.id"
        class="cyber-card-hover group cursor-pointer"
        @click="router.push(`/skills/${skill.id}`)"
      >
        <div class="flex items-start justify-between mb-3">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center">
              <Sparkles class="w-6 h-6 text-cyber-primary" />
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
            :class="getSyncStatus(skill.tools).color === 'success' ? 'bg-cyber-success/20 text-cyber-success' :
                   getSyncStatus(skill.tools).color === 'error' ? 'bg-cyber-error/20 text-cyber-error' :
                   getSyncStatus(skill.tools).color === 'warning' ? 'bg-cyber-warning/20 text-cyber-warning' :
                   'bg-cyber-muted/20 text-cyber-muted'"
          >
            {{ getSyncStatus(skill.tools).label }}
          </span>
        </div>
        
        <p class="text-sm text-cyber-muted mb-4 line-clamp-2">
          {{ skill.description }}
        </p>

        <div class="flex flex-wrap gap-1 mb-4">
          <span 
            v-for="tag in skill.tags" 
            :key="tag"
            class="px-2 py-0.5 rounded bg-cyber-border text-xs text-cyber-muted"
          >
            {{ tag }}
          </span>
        </div>

        <div class="flex items-center justify-between pt-3 border-t border-cyber-border">
          <div class="flex items-center gap-1">
            <span 
              v-for="tool in skill.tools.filter(t => t.installed).slice(0, 4)" 
              :key="tool.toolId"
              class="w-6 h-6 rounded flex items-center justify-center text-xs bg-cyber-card"
              :class="tool.conflict ? 'text-cyber-error' : 'text-cyber-muted'"
              :title="tool.toolId"
            >
              {{ tool.toolId.charAt(0).toUpperCase() }}
            </span>
          </div>
          
          <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button 
              class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary"
              @click.stop
            >
              <Edit class="w-4 h-4" />
            </button>
            <button 
              class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary"
              @click.stop
            >
              <Copy class="w-4 h-4" />
            </button>
            <button 
              class="p-1.5 rounded hover:bg-cyber-error/20 text-cyber-muted hover:text-cyber-error"
              @click.stop="skillStore.deleteSkill(skill.id)"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="skillStore.filteredSkills.length === 0" class="text-center py-12">
      <Sparkles class="w-12 h-12 text-cyber-muted mx-auto mb-4" />
      <h3 class="text-lg font-medium text-cyber-text mb-2">No skills found</h3>
      <p class="text-cyber-muted mb-4">Try adjusting your search or filters</p>
      <button 
        class="cyber-btn-primary inline-flex items-center gap-2"
        @click="showAddModal = true"
      >
        <Plus class="w-4 h-4" />
        <span>Add Your First Skill</span>
      </button>
    </div>

    <Teleport to="body">
      <transition name="modal">
        <div 
          v-if="showAddModal" 
          class="fixed inset-0 z-50 flex items-center justify-center p-4"
        >
          <div 
            class="absolute inset-0 bg-black/60 backdrop-blur-sm"
            @click="showAddModal = false"
          ></div>
          <div class="relative cyber-card w-full max-w-md p-6">
            <h2 class="text-xl font-semibold text-cyber-text mb-6">Add New Skill</h2>
            
            <form @submit.prevent="handleAddSkill" class="space-y-4">
              <div>
                <label class="block text-sm text-cyber-muted mb-2">Skill Name</label>
                <input 
                  v-model="newSkill.name"
                  type="text"
                  placeholder="e.g., vue-expert"
                  class="cyber-input w-full"
                  required
                />
              </div>
              
              <div>
                <label class="block text-sm text-cyber-muted mb-2">Description</label>
                <textarea 
                  v-model="newSkill.description"
                  placeholder="Describe what this skill does..."
                  rows="3"
                  class="cyber-input w-full resize-none"
                ></textarea>
              </div>
              
              <div>
                <label class="block text-sm text-cyber-muted mb-2">Version</label>
                <input 
                  v-model="newSkill.version"
                  type="text"
                  placeholder="1.0.0"
                  class="cyber-input w-full"
                />
              </div>
              
              <div>
                <label class="block text-sm text-cyber-muted mb-2">Tags (comma separated)</label>
                <input 
                  v-model="newSkill.tags"
                  type="text"
                  placeholder="e.g., vue, frontend, typescript"
                  class="cyber-input w-full"
                />
              </div>

              <div class="flex justify-end gap-3 pt-4">
                <button 
                  type="button"
                  class="cyber-btn bg-cyber-card text-cyber-muted"
                  @click="showAddModal = false"
                >
                  Cancel
                </button>
                <button 
                  type="submit"
                  class="cyber-btn-primary"
                >
                  Add Skill
                </button>
              </div>
            </form>
          </div>
        </div>
      </transition>
    </Teleport>
  </div>
</template>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
