<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import type { UniqueSkill } from '../../types'
import {
  Plus,
  Search,
  Trash2,
  FolderOpen,
  Globe,
  Wrench,
  X,
  Sparkles,
  ToggleRight,
  Loader2,
  RefreshCw,
} from 'lucide-vue-next'

const router = useRouter()
const skillStore = useSkillStore()

const showAddModal = ref(false)
const showDeleteConfirm = ref<UniqueSkill | null>(null)
const actionLoading = ref(false)
const actionError = ref<string | null>(null)

// Per-skill toggling state: "skillName:toolId"
const togglingKey = ref<string | null>(null)
const toggleErrors = ref<Record<string, string>>({})

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

const getSourceBadge = (skill: UniqueSkill) => {
  if (skill.sources.includes('global-agent')) {
    return { label: '全局代理', class: 'bg-cyber-secondary/20 text-cyber-secondary border-cyber-secondary/30' }
  }
  return { label: '工具专属', class: 'bg-cyber-primary/20 text-cyber-primary border-cyber-primary/30' }
}

// Toggle tool for a specific skill
const handleToggleTool = async (skill: UniqueSkill, toolId: string, event: MouseEvent) => {
  event.stopPropagation()
  const key = `${skill.name}:${toolId}`
  if (togglingKey.value === key) return

  togglingKey.value = key
  delete toggleErrors.value[key]

  try {
    // Use enabledTools (which accounts for global-agent inheritance) to determine current state
    const isEnabled = skill.enabledTools.includes(toolId)
    if (isEnabled) {
      await skillStore.disableSkillForTool(skill.name, toolId)
    } else {
      await skillStore.enableSkillForTool(skill.primaryPath, toolId)
    }
  } catch (e: any) {
    toggleErrors.value[key] = e.message || '操作失败'
    setTimeout(() => { delete toggleErrors.value[key] }, 3000)
  } finally {
    togglingKey.value = null
  }
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
    </div>

    <!-- Skills list -->
    <div class="space-y-3">
      <div
        v-for="skill in displayedSkills"
        :key="skill.name"
        class="cyber-card group cursor-pointer hover:border-cyber-primary/50 hover:shadow-[0_0_30px_rgba(0,245,255,0.08)] transition-all duration-300"
        @click="router.push(`/skills/${encodeURIComponent(skill.name)}`)"
      >
        <div class="flex items-start gap-4">
          <!-- Icon -->
          <div class="w-11 h-11 rounded-xl bg-gradient-to-br from-cyber-primary/20 to-cyber-secondary/20 border border-cyber-border flex items-center justify-center shrink-0 mt-0.5">
            <span class="text-cyber-primary font-bold text-lg">{{ skill.name.charAt(0).toUpperCase() }}</span>
          </div>

          <!-- Content -->
          <div class="flex-1 min-w-0">
            <!-- Top row: name + source badge + actions -->
            <div class="flex items-center justify-between gap-3 mb-1">
              <div class="flex items-center gap-2 min-w-0">
                <h3 class="font-semibold text-cyber-text group-hover:text-cyber-primary transition-colors truncate">
                  {{ skill.name }}
                </h3>
                <span
                  class="px-2 py-0.5 rounded-full text-[10px] font-medium border shrink-0"
                  :class="getSourceBadge(skill).class"
                >
                  {{ getSourceBadge(skill).label }}
                </span>
              </div>

              <!-- Actions (visible on hover) -->
              <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
                <button
                  class="p-1.5 rounded-lg hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary transition-colors"
                  title="在文件夹中打开"
                  @click.stop="skillStore.openInExplorer(skill.primaryPath)"
                >
                  <FolderOpen class="w-4 h-4" />
                </button>
                <button
                  class="p-1.5 rounded-lg hover:bg-cyber-error/20 text-cyber-muted hover:text-cyber-error transition-colors"
                  title="删除"
                  @click.stop="showDeleteConfirm = skill"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </div>

            <!-- Source label -->
            <p class="text-xs text-cyber-muted mb-2">{{ skill.primarySourceLabel }}</p>

            <!-- Description -->
            <p v-if="skill.description" class="text-sm text-cyber-muted mb-3 line-clamp-2">
              {{ skill.description }}
            </p>
            <p v-else class="text-sm text-cyber-muted/40 mb-3 italic">暂无描述</p>

            <!-- Tool tags -->
            <div
              class="flex flex-wrap gap-1.5"
              @click.stop
            >
              <template v-if="skillStore.availableTools.filter(t => t.exists).length > 0">
                <button
                  v-for="tool in skillStore.availableTools.filter(t => t.exists)"
                  :key="tool.id"
                  class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-medium border transition-colors duration-200"
                  :class="[
                    skill.enabledTools.includes(tool.id)
                      ? 'bg-cyber-primary/15 border-cyber-primary/40 text-cyber-primary hover:bg-cyber-primary/25 cursor-pointer'
                      : 'bg-cyber-card border-cyber-border text-cyber-muted hover:border-cyber-primary/40 hover:text-cyber-text cursor-pointer'
                  ]"
                  :disabled="togglingKey === `${skill.name}:${tool.id}`"
                  :title="skill.enabledTools.includes(tool.id) ? '点击关闭' : '点击开启'"
                  @click="handleToggleTool(skill, tool.id, $event)"
                >
                  <span class="w-3 h-3 flex items-center justify-center shrink-0">
                    <Loader2
                      v-if="togglingKey === `${skill.name}:${tool.id}`"
                      class="w-3 h-3 animate-spin"
                    />
                    <ToggleRight
                      v-else-if="skill.enabledTools.includes(tool.id)"
                      class="w-3 h-3"
                    />
                  </span>
                  {{ tool.name }}
                </button>
              </template>
              <span v-else class="text-[11px] text-cyber-muted/50 italic">未发现已安装的工具</span>

              <!-- Toggle error -->
              <span
                v-for="(errMsg, errKey) in toggleErrors"
                v-show="errKey.startsWith(skill.name + ':')"
                :key="errKey"
                class="text-[10px] text-cyber-error bg-cyber-error/10 px-2 py-1 rounded-lg border border-cyber-error/20"
              >
                {{ errMsg }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="displayedSkills.length === 0 && !skillStore.loading" class="text-center py-12">
      <Sparkles class="w-12 h-12 text-cyber-muted mx-auto mb-4" />
      <!-- Has filters active → search/filter no result -->
      <template v-if="skillStore.searchQuery || categoryFilter !== 'all'">
        <h3 class="text-lg font-medium text-cyber-text mb-2">没有匹配的 Skills</h3>
        <p class="text-cyber-muted mb-4">尝试修改搜索词或切换分类筛选</p>
        <button
          class="cyber-btn inline-flex items-center gap-2 bg-cyber-card text-cyber-muted border border-cyber-border hover:border-cyber-primary/40"
          @click="skillStore.searchQuery = ''; categoryFilter = 'all'"
        >
          清除筛选条件
        </button>
      </template>
      <!-- No filters → truly no skills -->
      <template v-else>
        <h3 class="text-lg font-medium text-cyber-text mb-2">未发现 Skills</h3>
        <p class="text-cyber-muted mb-4">在已知工具的 skill 文件夹中未发现任何 skill</p>
        <button
          class="cyber-btn-primary inline-flex items-center gap-2"
          @click="showAddModal = true"
        >
          <Plus class="w-4 h-4" />
          <span>新建第一个 Skill</span>
        </button>
      </template>
    </div>

    <!-- Loading state -->
    <div v-if="skillStore.loading" class="flex items-center justify-center py-12">
      <RefreshCw class="w-6 h-6 text-cyber-primary animate-spin mr-3" />
      <span class="text-cyber-muted">扫描 skills 中...</span>
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
