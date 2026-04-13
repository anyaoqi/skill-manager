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
  Loader2,
} from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const skillStore = useSkillStore()

const skillName = computed(() => decodeURIComponent(route.params.name as string))
const skill = computed(() => skillStore.getUniqueSkillByName(skillName.value))

const showDeleteConfirm = ref(false)
const deleteLoading = ref(false)
const deleteError = ref<string | null>(null)
const fileContent = ref<string | null>(null)
const selectedFile = ref<string | null>(null)
const fileLoading = ref(false)

const isGlobalAgent = computed(() => skill.value?.sources.includes('global-agent'))

const handleDelete = async () => {
  if (!skill.value) return
  deleteLoading.value = true
  deleteError.value = null
  try {
    for (const instance of skill.value.instances) {
      await skillStore.deleteSkill(instance.path)
    }
    router.push('/skills')
  } catch (e: any) {
    deleteError.value = e.message || '删除失败'
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
  deleteError.value = null
})
</script>

<template>
  <div v-if="skill" class="max-w-4xl mx-auto space-y-6">
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
            class="px-2 py-1 rounded-full text-xs font-medium border shrink-0"
            :class="isGlobalAgent
              ? 'bg-cyber-secondary/20 text-cyber-secondary border-cyber-secondary/30'
              : 'bg-cyber-primary/20 text-cyber-primary border-cyber-primary/30'"
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

    <!-- Description card -->
    <div class="cyber-card">
      <h2 class="text-lg font-semibold text-cyber-text mb-4 flex items-center gap-2">
        <Globe v-if="isGlobalAgent" class="w-5 h-5 text-cyber-secondary" />
        <Wrench v-else class="w-5 h-5 text-cyber-primary" />
        描述
      </h2>
      <p v-if="skill.description" class="text-cyber-muted leading-relaxed">{{ skill.description }}</p>
      <p v-else class="text-cyber-muted/50 italic">暂无描述，请在 SKILL.md 中添加</p>

      <!-- Source info -->
      <div class="mt-4 pt-4 border-t border-cyber-border grid grid-cols-2 gap-4 text-sm">
        <div>
          <span class="text-cyber-muted block mb-1">来源</span>
          <div class="flex flex-wrap gap-1">
            <span
              v-for="source in skill.sources"
              :key="source"
              class="inline-block text-cyber-primary"
            >
              {{ source === 'global-agent' ? '全局代理' : (skillStore.tools.find(t => t.id === source)?.name || source) }}
            </span>
          </div>
        </div>
        <div>
          <span class="text-cyber-muted block mb-1">路径数量</span>
          <span class="text-cyber-text">{{ skill.instances.length }} 个位置</span>
        </div>
      </div>
    </div>

    <!-- Files card -->
    <div class="cyber-card">
      <h2 class="text-lg font-semibold text-cyber-text mb-4">文件</h2>
      <div class="space-y-2">
        <div
          v-for="file in skill.files"
          :key="file.path"
        >
          <div
            class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg transition-colors"
            :class="[
              file.is_dir
                ? 'cursor-default'
                : 'cursor-pointer',
              selectedFile === file.path
                ? 'bg-cyber-primary/10 border border-cyber-primary/30'
                : 'hover:bg-cyber-border/30 border border-transparent'
            ]"
            @click="!file.is_dir && handleViewFile(file.path)"
          >
            <component
              :is="file.is_dir ? Folder : FileText"
              class="w-5 h-5 shrink-0"
              :class="file.is_dir ? 'text-cyber-warning' : 'text-cyber-primary'"
            />
            <span class="text-cyber-text flex-1 truncate text-sm">{{ file.name }}</span>
            <span class="text-cyber-muted text-xs shrink-0">{{ file.is_dir ? '文件夹' : '文件' }}</span>
          </div>

          <!-- File content viewer -->
          <div
            v-if="selectedFile === file.path && fileContent !== null"
            class="mt-1 ml-8 p-4 rounded-lg bg-cyber-bg border border-cyber-border overflow-x-auto"
          >
            <div v-if="fileLoading" class="flex items-center gap-2 text-cyber-muted">
              <Loader2 class="w-4 h-4 animate-spin" />
              <span class="text-sm">加载中...</span>
            </div>
            <pre v-else class="text-sm text-cyber-text whitespace-pre-wrap font-mono leading-relaxed">{{ fileContent }}</pre>
          </div>
        </div>

        <p v-if="skill.files.length === 0" class="text-cyber-muted/50 italic text-sm text-center py-4">
          暂无文件
        </p>
      </div>
    </div>

    <!-- All instances (only shown when multiple sources) -->
    <div v-if="skill.instances.length > 1" class="cyber-card">
      <h2 class="text-lg font-semibold text-cyber-text mb-4">所有位置</h2>
      <div class="space-y-2">
        <div
          v-for="instance in skill.instances"
          :key="instance.id"
          class="flex items-center gap-3 p-3 rounded-lg bg-cyber-bg border border-cyber-border/50"
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
            class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary shrink-0 transition-colors"
            title="打开文件夹"
            @click="skillStore.openInExplorer(instance.path)"
          >
            <FolderOpen class="w-4 h-4" />
          </button>
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

            <div v-if="deleteError" class="text-cyber-error text-sm bg-cyber-error/10 p-3 rounded-lg mb-4">
              {{ deleteError }}
            </div>

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
