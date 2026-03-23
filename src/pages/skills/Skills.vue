<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import type { UniqueSkill } from '../../types'
import {
  Plus,
  Search,
  Grid,
  List,
  Trash2,
  FolderOpen,
  Globe,
  Wrench,
  X,
  Sparkles,
} from 'lucide-vue-next'

const router = useRouter()
const skillStore = useSkillStore()

const viewMode = ref<'grid' | 'list'>('grid')
const showAddModal = ref(false)
const showDeleteConfirm = ref<UniqueSkill | null>(null)
const actionLoading = ref(false)
const actionError = ref<string | null>(null)

// Filter: 'all' | 'global-agent' | 'tool-specific'
const categoryFilter = ref<'all' | 'global-agent' | 'tool-specific'>('all')

const newSkill = ref({
  name: '',
  description: '',
  target: 'global-agent',
})

const displayedSkills = computed(() => {
  let result = skillStore.filteredSkills
  if (categoryFilter.value === 'global-agent') {
    result = result.filter(s => s.sources.includes('global-agent'))
  } else if (categoryFilter.value === 'tool-specific') {
    result = result.filter(s => !s.sources.includes('global-agent'))
  }
  return result
})

const categoryOptions = computed(() => [
  { value: 'all' as const, label: '全部', count: skillStore.filteredSkills.length },
  { value: 'global-agent' as const, label: '全局代理', count: skillStore.filteredSkills.filter(s => s.sources.includes('global-agent')).length },
  { value: 'tool-specific' as const, label: '工具专属', count: skillStore.filteredSkills.filter(s => !s.sources.includes('global-agent')).length },
])

const targetOptions = computed(() => {
  const options = [{ id: 'global-agent', name: '全局代理 (.agents/skills)' }]
  for (const tool of skillStore.availableTools) {
    options.push({ id: tool.id, name: `${tool.name} (${tool.skill_path})` })
  }
  return options
})

const getToolName = (toolId: string) => {
  const tool = skillStore.tools.find(t => t.id === toolId)
  return tool?.name || toolId
}

const getSourceBadge = (skill: UniqueSkill) => {
  if (skill.sources.includes('global-agent')) {
    return { label: '全局代理', class: 'bg-cyber-secondary/20 text-cyber-secondary' }
  }
  return { label: '工具专属', class: 'bg-cyber-primary/20 text-cyber-primary' }
}

const handleAddSkill = async () => {
  if (!newSkill.value.name.trim()) return
  actionLoading.value = true
  actionError.value = null
  try {
    await skillStore.createSkill(
      newSkill.value.target,
      newSkill.value.name.trim(),
      newSkill.value.description.trim()
    )
    newSkill.value = { name: '', description: '', target: 'global-agent' }
    showAddModal.value = false
  } catch (e: any) {
    actionError.value = e.message || '创建失败'
  } finally {
    actionLoading.value = false
  }
}

const handleDelete = async (skill: UniqueSkill) => {
  actionLoading.value = true
  actionError.value = null
  try {
    // Delete all instances
    for (const instance of skill.instances) {
      await skillStore.deleteSkill(instance.path)
    }
    showDeleteConfirm.value = null
  } catch (e: any) {
    actionError.value = e.message || '删除失败'
  } finally {
    actionLoading.value = false
  }
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">Skills</h1>
        <p class="text-cyber-muted mt-1">管理你的 AI 编程 skills</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="showAddModal = true"
      >
        <Plus class="w-4 h-4" />
        <span>新建 Skill</span>
      </button>
    </div>

    <!-- Filters -->
    <div class="flex items-center gap-4 flex-wrap">
      <div class="flex-1 relative min-w-[200px]">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-cyber-muted" />
        <input 
          v-model="skillStore.searchQuery"
          type="text"
          placeholder="搜索 skills..."
          class="cyber-input w-full pl-10"
        />
      </div>

      <!-- Category filter -->
      <div class="flex items-center gap-2">
        <button
          v-for="opt in categoryOptions"
          :key="opt.value"
          class="px-3 py-1.5 rounded-lg text-sm transition-all flex items-center gap-1.5"
          :class="categoryFilter === opt.value 
            ? 'bg-cyber-primary/20 text-cyber-primary border border-cyber-primary/30' 
            : 'bg-cyber-card text-cyber-muted border border-cyber-border hover:border-cyber-primary/30'"
          @click="categoryFilter = opt.value"
        >
          <Globe v-if="opt.value === 'global-agent'" class="w-3 h-3" />
          <Wrench v-else-if="opt.value === 'tool-specific'" class="w-3 h-3" />
          {{ opt.label }}
          <span class="text-[10px] opacity-60">({{ opt.count }})</span>
        </button>
      </div>

      <!-- View mode -->
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

    <!-- Skills grid/list -->
    <div 
      class="grid gap-4"
      :class="viewMode === 'grid' ? 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3' : 'grid-cols-1'"
    >
      <div
        v-for="skill in displayedSkills"
        :key="skill.name"
        class="cyber-card-hover group cursor-pointer"
        @click="router.push(`/skills/${encodeURIComponent(skill.name)}`)"
      >
        <!-- Header -->
        <div class="flex items-start justify-between mb-3">
          <div class="flex items-center gap-3 min-w-0">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center shrink-0">
              <span class="text-cyber-primary font-bold">{{ skill.name.charAt(0).toUpperCase() }}</span>
            </div>
            <div class="min-w-0">
              <h3 class="font-semibold text-cyber-text group-hover:text-cyber-primary transition-colors truncate">
                {{ skill.name }}
              </h3>
              <p class="text-xs text-cyber-muted">
                来源: {{ skill.primarySourceLabel }}
              </p>
            </div>
          </div>
          <span 
            class="px-2 py-1 rounded-full text-[10px] font-medium shrink-0 ml-2"
            :class="getSourceBadge(skill).class"
          >
            {{ getSourceBadge(skill).label }}
          </span>
        </div>
        
        <!-- Description -->
        <p v-if="skill.description" class="text-sm text-cyber-muted mb-3 line-clamp-2">
          {{ skill.description }}
        </p>
        <p v-else class="text-sm text-cyber-muted/50 mb-3 italic">暂无描述</p>

        <!-- Enabled tools -->
        <div class="flex items-center justify-between pt-3 border-t border-cyber-border">
          <div class="flex items-center gap-1 flex-wrap">
            <span 
              v-for="toolId in skill.enabledTools.slice(0, 3)" 
              :key="toolId"
              class="px-2 py-0.5 rounded bg-cyber-success/10 text-cyber-success text-[10px]"
            >
              {{ getToolName(toolId) }}
            </span>
            <span 
              v-if="skill.enabledTools.length > 3"
              class="px-2 py-0.5 rounded bg-cyber-bg text-[10px] text-cyber-muted"
            >
              +{{ skill.enabledTools.length - 3 }}
            </span>
            <span 
              v-if="skill.enabledTools.length === 0"
              class="text-[10px] text-cyber-muted/50"
            >
              未启用任何工具
            </span>
          </div>
          
          <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
            <button 
              class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary"
              @click.stop="skillStore.openInExplorer(skill.primaryPath)"
              title="在文件夹中打开"
            >
              <FolderOpen class="w-4 h-4" />
            </button>
            <button 
              class="p-1.5 rounded hover:bg-cyber-error/20 text-cyber-muted hover:text-cyber-error"
              @click.stop="showDeleteConfirm = skill"
              title="删除"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="displayedSkills.length === 0 && !skillStore.loading" class="text-center py-12">
      <Sparkles class="w-12 h-12 text-cyber-muted mx-auto mb-4" />
      <h3 class="text-lg font-medium text-cyber-text mb-2">未发现 Skills</h3>
      <p class="text-cyber-muted mb-4">在已知工具的 skill 文件夹中未发现任何 skill</p>
      <button 
        class="cyber-btn-primary inline-flex items-center gap-2"
        @click="showAddModal = true"
      >
        <Plus class="w-4 h-4" />
        <span>新建第一个 Skill</span>
      </button>
    </div>

    <!-- Add Skill Modal -->
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
            <div class="flex items-center justify-between mb-6">
              <h2 class="text-xl font-semibold text-cyber-text">新建 Skill</h2>
              <button 
                class="p-1 rounded hover:bg-cyber-border text-cyber-muted"
                @click="showAddModal = false"
              >
                <X class="w-5 h-5" />
              </button>
            </div>
            
            <form @submit.prevent="handleAddSkill" class="space-y-4">
              <div>
                <label class="block text-sm text-cyber-muted mb-2">Skill 名称</label>
                <input 
                  v-model="newSkill.name"
                  type="text"
                  placeholder="例如: vue-expert"
                  class="cyber-input w-full"
                  required
                />
              </div>
              
              <div>
                <label class="block text-sm text-cyber-muted mb-2">描述</label>
                <textarea 
                  v-model="newSkill.description"
                  placeholder="描述这个 skill 的功能..."
                  rows="3"
                  class="cyber-input w-full resize-none"
                ></textarea>
              </div>
              
              <div>
                <label class="block text-sm text-cyber-muted mb-2">创建位置</label>
                <select v-model="newSkill.target" class="cyber-input w-full">
                  <option 
                    v-for="opt in targetOptions" 
                    :key="opt.id"
                    :value="opt.id"
                  >
                    {{ opt.name }}
                  </option>
                </select>
              </div>

              <div v-if="actionError" class="text-cyber-error text-sm bg-cyber-error/10 p-3 rounded-lg">
                {{ actionError }}
              </div>

              <div class="flex justify-end gap-3 pt-4">
                <button 
                  type="button"
                  class="cyber-btn bg-cyber-card text-cyber-muted"
                  @click="showAddModal = false"
                >
                  取消
                </button>
                <button 
                  type="submit"
                  class="cyber-btn-primary"
                  :disabled="actionLoading"
                >
                  {{ actionLoading ? '创建中...' : '创建' }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </transition>
    </Teleport>

    <!-- Delete Confirm Modal -->
    <Teleport to="body">
      <transition name="modal">
        <div 
          v-if="showDeleteConfirm" 
          class="fixed inset-0 z-50 flex items-center justify-center p-4"
        >
          <div 
            class="absolute inset-0 bg-black/60 backdrop-blur-sm"
            @click="showDeleteConfirm = null"
          ></div>
          <div class="relative cyber-card w-full max-w-sm p-6">
            <h2 class="text-lg font-semibold text-cyber-text mb-3">确认删除</h2>
            <p class="text-cyber-muted text-sm mb-2">
              确定要删除 <span class="text-cyber-error font-medium">{{ showDeleteConfirm.name }}</span> 吗？
            </p>
            <p class="text-cyber-muted text-xs mb-4">
              这将从所有来源（共 {{ showDeleteConfirm.instances.length }} 个位置）中删除此 skill，此操作不可撤销。
            </p>

            <div v-if="actionError" class="text-cyber-error text-sm bg-cyber-error/10 p-3 rounded-lg mb-4">
              {{ actionError }}
            </div>

            <div class="flex justify-end gap-3">
              <button 
                class="cyber-btn bg-cyber-card text-cyber-muted"
                @click="showDeleteConfirm = null"
              >
                取消
              </button>
              <button 
                class="cyber-btn bg-cyber-error/20 text-cyber-error border border-cyber-error/30 hover:bg-cyber-error/30"
                :disabled="actionLoading"
                @click="handleDelete(showDeleteConfirm!)"
              >
                {{ actionLoading ? '删除中...' : '确认删除' }}
              </button>
            </div>
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
