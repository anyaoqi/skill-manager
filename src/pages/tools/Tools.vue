<script setup lang="ts">
import { useSkillStore } from '../../stores/skillStore'
import {
  RefreshCw,
  FolderOpen,
  Check,
  X,
  Globe,
  Wrench,
} from 'lucide-vue-next'

const skillStore = useSkillStore()
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-cyber-text">工具</h1>
        <p class="text-cyber-muted mt-1">查看和管理已发现的 AI 编程工具</p>
      </div>
      <button 
        class="cyber-btn-primary flex items-center gap-2"
        @click="skillStore.scanAllSkills()"
        :disabled="skillStore.loading"
      >
        <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': skillStore.loading }" />
        <span>重新扫描</span>
      </button>
    </div>

    <!-- Global Agent section -->
    <div class="space-y-3">
      <h2 class="text-lg font-semibold text-cyber-text flex items-center gap-2">
        <Globe class="w-5 h-5 text-cyber-secondary" />
        全局代理
      </h2>
      <p class="text-sm text-cyber-muted">
        全局代理的 skill 存放在 <code class="text-cyber-primary bg-cyber-bg px-2 py-0.5 rounded">.agents/skills/</code> 目录中，
        支持此规范的工具会自动加载这些 skill。
      </p>
      
      <div v-if="skillStore.globalAgentTool" class="cyber-card-hover">
        <div class="flex items-start justify-between mb-3">
          <div class="flex items-center gap-3">
            <div class="w-14 h-14 rounded-xl bg-gradient-to-br from-cyber-secondary/30 to-cyber-primary/20 flex items-center justify-center">
              <Globe class="w-7 h-7 text-cyber-secondary" />
            </div>
            <div>
              <h3 class="font-semibold text-cyber-text text-lg">{{ skillStore.globalAgentTool.name }}</h3>
              <p class="text-sm text-cyber-muted">共享 skill 目录</p>
            </div>
          </div>
          <div 
            class="px-3 py-1 rounded-full text-xs font-medium flex items-center gap-1"
            :class="skillStore.globalAgentTool.exists 
              ? 'bg-cyber-success/20 text-cyber-success' 
              : 'bg-cyber-muted/20 text-cyber-muted'"
          >
            <component :is="skillStore.globalAgentTool.exists ? Check : X" class="w-3 h-3" />
            {{ skillStore.globalAgentTool.exists ? '已发现' : '未找到' }}
          </div>
        </div>

        <div class="space-y-2">
          <div class="flex items-center gap-2 text-sm">
            <span class="text-cyber-muted w-20 shrink-0">路径:</span>
            <code class="text-cyber-text bg-cyber-bg px-3 py-1.5 rounded flex-1 truncate text-xs">
              {{ skillStore.globalAgentTool.resolved_path }}
            </code>
            <button 
              v-if="skillStore.globalAgentTool.exists"
              class="p-1.5 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary shrink-0"
              @click="skillStore.openInExplorer(skillStore.globalAgentTool.resolved_path)"
            >
              <FolderOpen class="w-4 h-4" />
            </button>
          </div>
          <div class="flex items-center gap-2 text-sm">
            <span class="text-cyber-muted w-20 shrink-0">Skills:</span>
            <span class="text-cyber-text">{{ skillStore.globalAgentTool.skill_count }} 个</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Tool-specific section -->
    <div class="space-y-3">
      <h2 class="text-lg font-semibold text-cyber-text flex items-center gap-2">
        <Wrench class="w-5 h-5 text-cyber-primary" />
        开发工具
      </h2>
      <p class="text-sm text-cyber-muted">
        每个工具有自己独立的 skill 文件夹，可以通过 Skill 详情页面的开关来管理各工具的 skill 支持。
      </p>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="tool in skillStore.availableTools"
          :key="tool.id"
          class="cyber-card-hover"
        >
          <div class="flex items-start justify-between mb-4">
            <div class="flex items-center gap-3">
              <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-cyber-primary/30 to-cyber-secondary/20 flex items-center justify-center">
                <span class="text-cyber-primary font-bold text-lg">{{ tool.name.charAt(0) }}</span>
              </div>
              <div>
                <h3 class="font-semibold text-cyber-text">{{ tool.name }}</h3>
                <p class="text-xs text-cyber-muted">{{ tool.skill_count }} skills</p>
              </div>
            </div>
            <div 
              class="px-2 py-1 rounded-full text-xs font-medium flex items-center gap-1"
              :class="tool.exists 
                ? 'bg-cyber-success/20 text-cyber-success' 
                : 'bg-cyber-muted/20 text-cyber-muted'"
            >
              <component :is="tool.exists ? Check : X" class="w-3 h-3" />
              {{ tool.exists ? '已发现' : '未找到' }}
            </div>
          </div>

          <div class="space-y-2">
            <div>
              <label class="block text-xs text-cyber-muted mb-1">Skill 路径</label>
              <div class="flex items-center gap-2">
                <code class="text-xs text-cyber-text bg-cyber-bg px-3 py-1.5 rounded flex-1 truncate">
                  {{ tool.resolved_path }}
                </code>
                <button 
                  v-if="tool.exists"
                  class="p-1 rounded hover:bg-cyber-border text-cyber-muted hover:text-cyber-primary shrink-0"
                  @click="skillStore.openInExplorer(tool.resolved_path)"
                >
                  <FolderOpen class="w-4 h-4" />
                </button>
              </div>
            </div>

            <div v-if="tool.supports_global_agent" class="flex items-center gap-2 pt-2 border-t border-cyber-border">
              <Globe class="w-3 h-3 text-cyber-secondary" />
              <span class="text-xs text-cyber-secondary">支持全局代理</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Info -->
    <div class="cyber-card">
      <h2 class="text-lg font-semibold text-cyber-text mb-4">工具路径说明</h2>
      <p class="text-cyber-muted text-sm mb-4">
        SkillDock 自动扫描以下已知路径来发现本地安装的工具和 skill。
        上方显示的路径为自动检测的默认路径。
      </p>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-secondary mb-1 font-medium">全局代理</p>
          <code class="text-cyber-text text-xs">~/.agents/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">OpenCode</p>
          <code class="text-cyber-text text-xs">~/.config/opencode/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">Claude Code</p>
          <code class="text-cyber-text text-xs">~/.claude/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">Cursor</p>
          <code class="text-cyber-text text-xs">~/.cursor/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">VS Code (Copilot)</p>
          <code class="text-cyber-text text-xs">~/.vscode/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">Trae</p>
          <code class="text-cyber-text text-xs">~/.trae/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">Windsurf</p>
          <code class="text-cyber-text text-xs">~/.windsurf/skills/</code>
        </div>
        <div class="p-3 rounded-lg bg-cyber-bg">
          <p class="text-cyber-primary mb-1 font-medium">Augment Code</p>
          <code class="text-cyber-text text-xs">~/.augment/skills/</code>
        </div>
      </div>
    </div>
  </div>
</template>
