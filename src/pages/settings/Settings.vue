<script setup lang="ts">
import { useSettingsStore } from '../../stores/settingsStore'
import { useSkillStore } from '../../stores/skillStore'
import {
  Settings,
  Bell,
  RefreshCw,
  Moon,
  Sun,
  Monitor,
  Info,
} from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const skillStore = useSkillStore()
</script>

<template>
  <div class="space-y-6 max-w-2xl">
    <div>
      <h1 class="text-2xl font-bold text-cyber-text">设置</h1>
      <p class="text-cyber-muted mt-1">配置 SkillDock 偏好设置</p>
    </div>

    <div class="space-y-4">
      <!-- Appearance -->
      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">外观</h2>
        
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <Moon class="w-5 h-5 text-cyber-muted" />
            <div>
              <p class="text-cyber-text">主题</p>
              <p class="text-xs text-cyber-muted">选择你偏好的主题</p>
            </div>
          </div>
          <div class="flex items-center gap-1 bg-cyber-bg rounded-lg p-1">
            <button 
              class="p-2 rounded transition-colors"
              :class="settingsStore.settings.theme === 'dark' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              @click="settingsStore.updateSettings({ theme: 'dark' })"
            >
              <Moon class="w-4 h-4" />
            </button>
            <button 
              class="p-2 rounded transition-colors"
              :class="settingsStore.settings.theme === 'light' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              @click="settingsStore.updateSettings({ theme: 'light' })"
            >
              <Sun class="w-4 h-4" />
            </button>
            <button 
              class="p-2 rounded transition-colors"
              :class="settingsStore.settings.theme === 'system' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              @click="settingsStore.updateSettings({ theme: 'system' })"
            >
              <Monitor class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <!-- Scan -->
      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">扫描</h2>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <RefreshCw class="w-5 h-5 text-cyber-muted" />
              <div>
                <p class="text-cyber-text">启动时自动扫描</p>
                <p class="text-xs text-cyber-muted">应用启动时自动扫描所有工具的 skill 目录</p>
              </div>
            </div>
            <button 
              class="w-12 h-6 rounded-full transition-colors relative"
              :class="settingsStore.settings.autoScan ? 'bg-cyber-primary' : 'bg-cyber-border'"
              @click="settingsStore.updateSettings({ autoScan: !settingsStore.settings.autoScan })"
            >
              <span 
                class="absolute top-1 w-4 h-4 rounded-full bg-white transition-transform"
                :class="settingsStore.settings.autoScan ? 'left-7' : 'left-1'"
              ></span>
            </button>
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <RefreshCw class="w-5 h-5 text-cyber-muted" />
              <div>
                <p class="text-cyber-text">立即扫描</p>
                <p class="text-xs text-cyber-muted">手动刷新扫描所有工具和 skills</p>
              </div>
            </div>
            <button 
              class="cyber-btn-primary text-sm"
              :disabled="skillStore.loading"
              @click="skillStore.scanAllSkills()"
            >
              <RefreshCw class="w-4 h-4 inline mr-1" :class="{ 'animate-spin': skillStore.loading }" />
              {{ skillStore.loading ? '扫描中...' : '扫描' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Notifications -->
      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">通知</h2>
        
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <Bell class="w-5 h-5 text-cyber-muted" />
            <div>
              <p class="text-cyber-text">操作通知</p>
              <p class="text-xs text-cyber-muted">在操作完成后显示通知</p>
            </div>
          </div>
          <button 
            class="w-12 h-6 rounded-full transition-colors relative"
            :class="settingsStore.settings.notifications ? 'bg-cyber-primary' : 'bg-cyber-border'"
            @click="settingsStore.updateSettings({ notifications: !settingsStore.settings.notifications })"
          >
            <span 
              class="absolute top-1 w-4 h-4 rounded-full bg-white transition-transform"
              :class="settingsStore.settings.notifications ? 'left-7' : 'left-1'"
            ></span>
          </button>
        </div>
      </div>

      <!-- About -->
      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4 flex items-center gap-2">
          <Info class="w-5 h-5 text-cyber-muted" />
          关于
        </h2>
        
        <div class="space-y-3">
          <div class="flex items-center justify-between py-2 border-b border-cyber-border">
            <span class="text-cyber-muted">版本</span>
            <span class="text-cyber-text">0.1.0</span>
          </div>
          <div class="flex items-center justify-between py-2 border-b border-cyber-border">
            <span class="text-cyber-muted">技术栈</span>
            <span class="text-cyber-text">Tauri 2 + Vue 3</span>
          </div>
          <div class="flex items-center justify-between py-2 border-b border-cyber-border">
            <span class="text-cyber-muted">已发现工具</span>
            <span class="text-cyber-text">{{ skillStore.tools.filter(t => t.exists).length }} / {{ skillStore.tools.length }}</span>
          </div>
          <div class="flex items-center justify-between py-2">
            <span class="text-cyber-muted">License</span>
            <span class="text-cyber-text">MIT</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
