<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import {
  ArrowLeft,
  Trash2,
  Folder,
  FileText,
  FolderOpen,
  Globe,
  Wrench,
  AlertTriangle,
  ToggleLeft,
  ToggleRight,
  Loader2,
} from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const skillStore = useSkillStore()

const skillName = computed(() => decodeURIComponent(route.params.name as string))
const skill = computed(() => skillStore.getUniqueSkillByName(skillName.value))

const togglingTool = ref<string | null>(null)
const toggleError = ref<string | null>(null)
const showDeleteConfirm = ref(false)
const deleteLoading = ref(false)
const fileContent = ref<string | null>(null)
const selectedFile = ref<string | null>(null)
const fileLoading = ref(false)

const isGlobalAgent = computed(() => skill.value?.sources.includes('global-agent'))

const handleToggleTool = async (toolId: string) => {
  if (!skill.value) return
  togglingTool.value = toolId
  toggleError.value = null

  try {
    const isEnabled = skillStore.isSkillEnabledForTool(skill.value.name, toolId)
    
    if (isEnabled) {
      await skillStore.disableSkillForTool(skill.value.name, toolId)
    } else {
      // Use the primary path as source to copy from
      await skillStore.enableSkillForTool(skill.value.primaryPath, toolId)
    }
  } catch (e: any) {
    toggleError.value = e.message || '操作失败'
  } finally {
    togglingTool.value = null
  }
}

const handleDelete = async () => {
  if (!skill.value) return
  deleteLoading.value = true
  try {
    for (const instance of skill.value.instances) {
      await skillStore.deleteSkill(instance.path)
    }
    router.push('/skills')
  } catch (e: any) {
    toggleError.value = e.message || '删除失败'
  } finally {
    deleteLoading.value = false
  }
}

const handleViewFile = async (filePath: string) => {
  if (selectedFile.value === filePath) {
    selectedFile.value = null
    fileContent.value = null
    return
  }
  fileLoading.value = true
  selectedFile.value = filePath
  try {
    fileContent.value = await skillStore.readSkillFile(filePath)
  } catch (e: any) {
    fileContent.value = `无法读取文件: ${e.message}`
  } finally {
    fileLoading.value = false
  }
}

// Reset state when navigating to different skill
watch(skillName, () => {
  fileContent.value = null
  selectedFile.value = null
  toggleError.value = null
})
</script>

<template>
  <div v-if="skill" class="space-y-6">
    <!-- Header -->
    <div class="flex items-center gap-4">
      <button 
        class="p-2 rounded-lg hover:bg-cyber-card text-cyber-muted hover:text-cyber-text transition-colors"
        @click="router.back()"
      >
        <ArrowLeft class="w-5 h-5" />
      </button>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-3">
          <h1 class="text-2xl font-bold text-cyber-text truncate">{{ skill.name }}</h1>
          <span 
            class="px-2 py-1 rounded-full text-xs font-medium shrink-0"
            :class="isGlobalAgent 
              ? 'bg-cyber-secondary/20 text-cyber-secondary' 
              : 'bg-cyber-primary/20 text-cyber-primary'"
          >
            {{ isGlobalAgent ? '全局代理' : '工具专属' }}
          </span>
        </div>
        <p class="text-cyber-muted text-sm mt-1 truncate">{{ skill.primaryPath }}</p>
      </div>
      <button 
        class="cyber-btn bg-cyber-card text-cyber-muted hover:text-cyber-primary flex items-center gap-2"
        @click="skillStore.openInExplorer(skill.primaryPath)"
      >
        <FolderOpen class="w-4 h-4" />
        <span>打开文件夹</span>
      </button>
      <button 
        class="cyber-btn bg-cyber-error/10 text-cyber-error border border-cyber-error/30 hover:bg-cyber-error/20 flex items-center gap-2"
        @click="showDeleteConfirm = true"
      >
        <Trash2 class="w-4 h-4" />
        <span>删除</span>
      </button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left column -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Description -->
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4 flex items-center gap-2">
            <Globe v-if="isGlobalAgent" class="w-5 h-5 text-cyber-secondary" />
            <Wrench v-else class="w-5 h-5 text-cyber-primary" />
            描述
          </h2>
          <p v-if="skill.description" class="text-cyber-muted">{{ skill.description }}</p>
          <p v-else class="text-cyber-muted/50 italic">暂无描述，请在 SKILL.md 中添加</p>
          
          <!-- Source info -->
          <div class="mt-4 pt-4 border-t border-cyber-border">
            <p class="text-sm text-cyber-muted">
              <span class="text-cyber-text">来源：</span>
              <span 
                v-for="(source, index) in skill.sources" 
                :key="source"
              >
                <span class="text-cyber-primary">{{ 
                  skillStore.tools.find(t => t.id === source)?.name || source 
                }}</span>
                <span v-if="index < skill.sources.length - 1">, </span>
              </span>
            </p>
            <p class="text-sm text-cyber-muted mt-1">
              <span class="text-cyber-text">路径数：</span>
              {{ skill.instances.length }} 个位置
            </p>
          </div>
        </div>

        <!-- Files -->
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">文件</h2>
          <div class="space-y-2">
            <div 
              v-for="file in skill.files" 
              :key="file.path"
            >
              <div 
                class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg cursor-pointer transition-colors"
                :class="selectedFile === file.path 
                  ? 'bg-cyber-primary/10 border border-cyber-primary/30' 
                  : 'hover:bg-cyber-border/30'"
                @click="!file.is_dir && handleViewFile(file.path)"
              >
                <component 
                  :is="file.is_dir ? Folder : FileText" 
                  class="w-5 h-5 text-cyber-primary shrink-0"
                />
                <span class="text-cyber-text flex-1 truncate">{{ file.name }}</span>
                <span class="text-cyber-muted text-xs shrink-0">{{ file.is_dir ? '文件夹' : '文件' }}</span>
              </div>
              
              <!-- File content viewer -->
              <div 
                v-if="selectedFile === file.path && fileContent !== null"
                class="mt-1 ml-8 p-4 rounded-lg bg-cyber-bg border border-cyber-border overflow-x-auto"
              >
                <div v-if="fileLoading" class="flex items-center gap-2 text-cyber-muted">
                  <Loader2 class="w-4 h-4 animate-spin" />
                  <span>加载中...</span>
                </div>
                <pre v-else class="text-sm text-cyber-text whitespace-pre-wrap font-mono leading-relaxed">{{ fileContent }}</pre>
              </div>
            </div>
          </div>
        </div>

        <!-- All instances -->
        <div v-if="skill.instances.length > 1" class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-4">所有位置</h2>
          <div class="space-y-2">
            <div 
              v-for="instance in skill.instances" 
              :key="instance.id"
              class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg"
            >
              <div 
                class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0"
                :class="instance.source === 'global-agent' 
                  ? 'bg-cyber-secondary/20 text-cyber-secondary' 
                  : 'bg-cyber-primary/20 text-cyber-primary'"
              >
                <Globe v-if="instance.source === 'global-agent'" class="w-4 h-4" />
                <Wrench v-else class="w-4 h-4" />
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm text-cyber-text">{{ instance.source_label }}</p>
                <p class="text-xs text-cyber-muted truncate">{{ instance.path }}</p>
              </div>
              <button 
                class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary shrink-0"
                @click="skillStore.openInExplorer(instance.path)"
                title="打开文件夹"
              >
                <FolderOpen class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Right column: Tool toggles -->
      <div class="space-y-6">
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-2">工具支持</h2>
          <p class="text-sm text-cyber-muted mb-4">
            {{ isGlobalAgent 
              ? '此 skill 位于全局代理目录，支持全局代理的工具将自动加载'
              : '一键开启或关闭此 skill 对各工具的支持' }}
          </p>

          <!-- Error message -->
          <div v-if="toggleError" class="text-cyber-error text-sm bg-cyber-error/10 p-3 rounded-lg mb-4">
            {{ toggleError }}
          </div>

          <!-- Auto-supported tools (only for global-agent skills) -->
          <div v-if="isGlobalAgent" class="mb-4">
            <p class="text-xs text-cyber-secondary font-medium mb-2 flex items-center gap-1.5">
              <Lock class="w-3 h-3" />
              自动支持的工具
            </p>
            <div class="space-y-2">
              <div 
                v-for="tool in skillStore.availableTools.filter(t => t.supports_global_agent)" 
                :key="tool.id"
                class="flex items-center justify-between p-3 rounded-lg bg-cyber-bg border border-cyber-secondary/20"
              >
                <div class="flex items-center gap-3">
                  <div class="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-medium bg-cyber-success/20 text-cyber-success">
                    {{ tool.name.charAt(0) }}
                  </div>
                  <div>
                    <p class="text-cyber-text font-medium text-sm">{{ tool.name }}</p>
                    <p class="text-[10px] text-cyber-secondary">自动加载全局代理 skill</p>
                  </div>
                </div>
                
                <span class="flex items-center gap-1 text-[10px] text-cyber-secondary bg-cyber-secondary/10 px-2 py-1 rounded-full">
                  <Lock class="w-3 h-3" />
                  自动支持
                </span>
              </div>
            </div>
          </div>
          
          <!-- Toggleable tools -->
          <div>
            <p v-if="isGlobalAgent" class="text-xs text-cyber-muted font-medium mb-2 flex items-center gap-1.5">
              <Wrench class="w-3 h-3" />
              其他工具（需手动开启）
            </p>
            <div class="space-y-2">
              <div 
                v-for="tool in isGlobalAgent 
                  ? skillStore.availableTools.filter(t => !t.supports_global_agent)
                  : skillStore.availableTools" 
                :key="tool.id"
                class="flex items-center justify-between p-3 rounded-lg bg-cyber-bg"
              >
                <div class="flex items-center gap-3">
                  <div 
                    class="w-8 h-8 rounded-lg flex items-center justify-center text-sm font-medium"
                    :class="skillStore.isSkillEnabledForTool(skill.name, tool.id) 
                      ? 'bg-cyber-success/20 text-cyber-success' 
                      : 'bg-cyber-muted/20 text-cyber-muted'"
                  >
                    {{ tool.name.charAt(0) }}
                  </div>
                  <div>
                    <p class="text-cyber-text font-medium text-sm">{{ tool.name }}</p>
                    <p class="text-[10px] text-cyber-muted">{{ tool.skill_path }}</p>
                  </div>
                </div>
                
                <button
                  class="flex items-center transition-colors"
                  :disabled="togglingTool === tool.id || !tool.exists"
                  @click="handleToggleTool(tool.id)"
                  :title="!tool.exists ? '工具未发现' : (skillStore.isSkillEnabledForTool(skill.name, tool.id) ? '点击关闭' : '点击开启')"
                >
                  <Loader2 
                    v-if="togglingTool === tool.id" 
                    class="w-6 h-6 text-cyber-primary animate-spin" 
                  />
                  <ToggleRight 
                    v-else-if="skillStore.isSkillEnabledForTool(skill.name, tool.id)" 
                    class="w-8 h-8 text-cyber-success cursor-pointer hover:text-cyber-success/80 transition-colors" 
                  />
                  <ToggleLeft 
                    v-else 
                    class="w-8 h-8 cursor-pointer transition-colors" 
                    :class="tool.exists ? 'text-cyber-muted hover:text-cyber-primary' : 'text-cyber-muted/30 cursor-not-allowed'"
                  />
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Global agent status -->
        <div class="cyber-card">
          <h2 class="text-lg font-semibold text-cyber-text mb-3 flex items-center gap-2">
            <Globe class="w-5 h-5 text-cyber-secondary" />
            全局代理
          </h2>
          <div class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg">
            <div 
              class="w-8 h-8 rounded-lg flex items-center justify-center"
              :class="isGlobalAgent 
                ? 'bg-cyber-success/20 text-cyber-success' 
                : 'bg-cyber-muted/20 text-cyber-muted'"
            >
              {{ isGlobalAgent ? '✓' : '✗' }}
            </div>
            <div>
              <p class="text-sm text-cyber-text">
                {{ isGlobalAgent ? '已在全局代理中' : '不在全局代理中' }}
              </p>
              <p class="text-[10px] text-cyber-muted">.agents/skills/</p>
            </div>
          </div>
          <p class="text-xs text-cyber-muted mt-3">
            全局代理 skill 可被所有支持 .agents 规范的工具共享
          </p>
        </div>
      </div>
    </div>

    <!-- Delete Confirm Modal -->
    <Teleport to="body">
      <transition name="modal">
        <div 
          v-if="showDeleteConfirm" 
          class="fixed inset-0 z-50 flex items-center justify-center p-4"
        >
          <div 
            class="absolute inset-0 bg-black/60 backdrop-blur-sm"
            @click="showDeleteConfirm = false"
          ></div>
          <div class="relative cyber-card w-full max-w-sm p-6">
            <h2 class="text-lg font-semibold text-cyber-text mb-3">确认删除</h2>
            <p class="text-cyber-muted text-sm mb-2">
              确定要删除 <span class="text-cyber-error font-medium">{{ skill.name }}</span> 吗？
            </p>
            <p class="text-cyber-muted text-xs mb-4">
              这将从所有来源（共 {{ skill.instances.length }} 个位置）中删除此 skill，此操作不可撤销。
            </p>

            <div class="flex justify-end gap-3">
              <button 
                class="cyber-btn bg-cyber-card text-cyber-muted"
                @click="showDeleteConfirm = false"
              >
                取消
              </button>
              <button 
                class="cyber-btn bg-cyber-error/20 text-cyber-error border border-cyber-error/30 hover:bg-cyber-error/30"
                :disabled="deleteLoading"
                @click="handleDelete()"
              >
                {{ deleteLoading ? '删除中...' : '确认删除' }}
              </button>
            </div>
          </div>
        </div>
      </transition>
    </Teleport>
  </div>

  <!-- Skill not found -->
  <div v-else class="flex items-center justify-center h-64">
    <div class="text-center">
      <AlertTriangle class="w-12 h-12 text-cyber-warning mx-auto mb-4" />
      <h2 class="text-xl font-semibold text-cyber-text mb-2">Skill 未找到</h2>
      <p class="text-cyber-muted mb-4">找不到名为 "{{ skillName }}" 的 skill</p>
      <button 
        class="cyber-btn-primary"
        @click="router.push('/skills')"
      >
        返回 Skills
      </button>
    </div>
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
