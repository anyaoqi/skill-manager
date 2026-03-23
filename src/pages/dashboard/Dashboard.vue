<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useSkillStore } from '../../stores/skillStore'
import {
  Boxes,
  Globe,
  Wrench,
  ArrowRight,
  Sparkles,
  FolderOpen,
  AlertCircle,
} from 'lucide-vue-next'

const router = useRouter()
const skillStore = useSkillStore()

const stats = computed(() => [
  {
    label: '全部 Skills',
    value: skillStore.totalSkillCount,
    icon: Boxes,
    color: 'primary',
  },
  {
    label: '全局代理 Skills',
    value: skillStore.globalSkillCount,
    icon: Globe,
    color: 'secondary',
  },
  {
    label: '工具独有 Skills',
    value: skillStore.toolSpecificCount,
    icon: Wrench,
    color: 'success',
  },
  {
    label: '已发现工具',
    value: skillStore.tools.filter(t => t.exists).length,
    icon: FolderOpen,
    color: 'warning',
  },
])

const recentSkills = computed(() => skillStore.uniqueSkills.slice(0, 6))

const getToolName = (toolId: string) => {
  const tool = skillStore.tools.find(t => t.id === toolId)
  return tool?.name || toolId
}

const getColorClass = (color: string) => {
  const map: Record<string, string> = {
    primary: 'text-cyber-primary',
    secondary: 'text-cyber-secondary',
    success: 'text-cyber-success',
    warning: 'text-cyber-warning',
    error: 'text-cyber-error',
    muted: 'text-cyber-muted',
  }
  return map[color] || 'text-cyber-muted'
}

const getBgClass = (color: string) => {
  const map: Record<string, string> = {
    primary: 'from-cyber-primary/20 to-cyber-primary/5',
    secondary: 'from-cyber-secondary/20 to-cyber-secondary/5',
    success: 'from-cyber-success/20 to-cyber-success/5',
    warning: 'from-cyber-warning/20 to-cyber-warning/5',
  }
  return map[color] || 'from-cyber-border to-cyber-border/50'
}

const getSourceBadge = (skill: typeof recentSkills.value[0]) => {
  if (skill.sources.includes('global-agent')) {
    return { label: '全局代理', class: 'bg-cyber-secondary/20 text-cyber-secondary' }
  }
  return { label: '工具专属', class: 'bg-cyber-primary/20 text-cyber-primary' }
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">总览</h1>
        <p class="text-cyber-muted mt-1">管理本地 AI 编程工具的 Skills</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="router.push('/skills')"
      >
        <Sparkles class="w-4 h-4" />
        <span>查看全部</span>
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="skillStore.loading" class="flex items-center justify-center py-12">
      <div class="flex items-center gap-3 text-cyber-muted">
        <div class="w-5 h-5 border-2 border-cyber-primary border-t-transparent rounded-full animate-spin"></div>
        <span>正在扫描本地 skill 文件夹...</span>
      </div>
    </div>

    <!-- Error state -->
    <div v-else-if="skillStore.error" class="cyber-card text-center py-8">
      <AlertCircle class="w-12 h-12 text-cyber-error mx-auto mb-4" />
      <p class="text-cyber-error">{{ skillStore.error }}</p>
      <button class="cyber-btn-primary mt-4" @click="skillStore.scanAllSkills()">重试</button>
    </div>

    <template v-else>
      <!-- Stats -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <div
          v-for="stat in stats"
          :key="stat.label"
          class="cyber-card-hover group"
        >
          <div class="flex items-start justify-between">
            <div>
              <p class="text-cyber-muted text-sm">{{ stat.label }}</p>
              <p class="text-3xl font-bold mt-2" :class="getColorClass(stat.color)">
                {{ stat.value }}
              </p>
            </div>
            <div 
              class="w-12 h-12 rounded-xl bg-gradient-to-br flex items-center justify-center transition-transform group-hover:scale-110"
              :class="getBgClass(stat.color)"
            >
              <component :is="stat.icon" class="w-6 h-6" :class="getColorClass(stat.color)" />
            </div>
          </div>
        </div>
      </div>

      <!-- Main content -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Skills list -->
        <div class="lg:col-span-2 space-y-4">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-cyber-text">Skills 概览</h2>
            <button 
              class="text-cyber-primary text-sm flex items-center gap-1 hover:gap-2 transition-all"
              @click="router.push('/skills')"
            >
              查看全部 <ArrowRight class="w-4 h-4" />
            </button>
          </div>

          <div v-if="recentSkills.length === 0" class="cyber-card text-center py-8">
            <Boxes class="w-12 h-12 text-cyber-muted mx-auto mb-4" />
            <h3 class="text-lg font-medium text-cyber-text mb-2">暂未发现 Skills</h3>
            <p class="text-cyber-muted">请检查工具的 skill 文件夹路径是否正确</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div
              v-for="skill in recentSkills"
              :key="skill.name"
              class="cyber-card-hover cursor-pointer group"
              @click="router.push(`/skills/${encodeURIComponent(skill.name)}`)"
            >
              <div class="flex items-start justify-between mb-3">
                <div class="flex items-center gap-2">
                  <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/30 flex items-center justify-center">
                    <span class="text-cyber-primary font-bold text-sm">{{ skill.name.charAt(0).toUpperCase() }}</span>
                  </div>
                  <div>
                    <h3 class="font-semibold text-cyber-text group-hover:text-cyber-primary transition-colors text-sm">
                      {{ skill.name }}
                    </h3>
                    <p class="text-xs text-cyber-muted">
                      {{ skill.enabledTools.length }} 个工具已启用
                    </p>
                  </div>
                </div>
                <span 
                  class="px-2 py-0.5 rounded-full text-[10px] font-medium shrink-0"
                  :class="getSourceBadge(skill).class"
                >
                  {{ getSourceBadge(skill).label }}
                </span>
              </div>
              
              <p v-if="skill.description" class="text-xs text-cyber-muted line-clamp-2 mb-3">
                {{ skill.description }}
              </p>

              <!-- Tool badges -->
              <div class="flex flex-wrap gap-1 pt-2 border-t border-cyber-border">
                <span 
                  v-for="toolId in skill.enabledTools.slice(0, 4)" 
                  :key="toolId"
                  class="px-2 py-0.5 rounded bg-cyber-bg text-[10px] text-cyber-muted"
                >
                  {{ getToolName(toolId) }}
                </span>
                <span 
                  v-if="skill.enabledTools.length > 4"
                  class="px-2 py-0.5 rounded bg-cyber-bg text-[10px] text-cyber-muted"
                >
                  +{{ skill.enabledTools.length - 4 }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Tools sidebar -->
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-cyber-text">工具状态</h2>
            <button 
              class="text-cyber-primary text-sm flex items-center gap-1 hover:gap-2 transition-all"
              @click="router.push('/tools')"
            >
              管理 <ArrowRight class="w-4 h-4" />
            </button>
          </div>

          <div class="space-y-2">
            <div
              v-for="tool in skillStore.tools"
              :key="tool.id"
              class="cyber-card flex items-center gap-3 py-3"
            >
              <div 
                class="w-2 h-2 rounded-full shrink-0"
                :class="tool.exists ? 'bg-cyber-success' : 'bg-cyber-muted'"
              ></div>
              <div class="flex-1 min-w-0">
                <h3 class="font-medium text-cyber-text text-sm truncate">{{ tool.name }}</h3>
                <p class="text-[10px] text-cyber-muted truncate">{{ tool.skill_count }} skills</p>
              </div>
              <span 
                class="text-[10px] px-2 py-0.5 rounded-full shrink-0"
                :class="tool.exists 
                  ? 'bg-cyber-success/20 text-cyber-success' 
                  : 'bg-cyber-muted/20 text-cyber-muted'"
              >
                {{ tool.exists ? '已发现' : '未找到' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
