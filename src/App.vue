<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useSkillStore } from './stores/skillStore'
import { useSettingsStore } from './stores/settingsStore'
import {
  LayoutDashboard,
  Boxes,
  Wrench,
  Settings,
  Command,
  RefreshCw,
} from 'lucide-vue-next'

const route = useRoute()
const skillStore = useSkillStore()
const settingsStore = useSettingsStore()

const navItems = [
  { path: '/', name: '总览', icon: LayoutDashboard },
  { path: '/skills', name: 'Skills', icon: Boxes },
  { path: '/tools', name: '工具', icon: Wrench },
  { path: '/settings', name: '设置', icon: Settings },
]

const isActive = (path: string) => {
  if (path === '/') return route.path === '/'
  return route.path.startsWith(path)
}

const applyTheme = (theme: string) => {
  const root = document.documentElement
  root.classList.remove('theme-light', 'theme-dark', 'theme-system')
  root.classList.add(`theme-${theme}`)
}

// Apply theme on change
watch(
  () => settingsStore.settings.theme,
  (theme) => applyTheme(theme),
  { immediate: true }
)

onMounted(() => {
  // Respect the autoScan setting
  if (settingsStore.settings.autoScan) {
    skillStore.scanAllSkills()
  }
})
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-cyber-bg overflow-hidden">
    <header class="h-14 flex items-center justify-between px-4 border-b border-cyber-border glass" data-tauri-drag-region>
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-cyber-primary to-cyber-secondary flex items-center justify-center">
          <Command class="w-5 h-5 text-white" />
        </div>
        <span class="text-lg font-semibold gradient-text">SkillDock</span>
      </div>

      <button
        class="p-2 rounded-lg hover:bg-cyber-card transition-colors text-cyber-muted hover:text-cyber-primary"
        :class="{ 'animate-spin': skillStore.loading }"
        :disabled="skillStore.loading"
        @click="skillStore.scanAllSkills()"
        title="刷新扫描"
      >
        <RefreshCw class="w-5 h-5" />
      </button>
    </header>

    <div class="flex-1 flex overflow-hidden">
      <nav class="w-16 flex flex-col items-center py-4 border-r border-cyber-border glass">
        <div class="flex flex-col gap-2 w-full px-2">
          <router-link
            v-for="item in navItems"
            :key="item.path"
            :to="item.path"
            class="flex flex-col items-center gap-1 py-2 px-2 rounded-lg transition-all duration-200 group"
            :class="isActive(item.path) 
              ? 'bg-cyber-primary/10 text-cyber-primary' 
              : 'text-cyber-muted hover:text-cyber-text hover:bg-cyber-card'"
          >
            <component 
              :is="item.icon" 
              class="w-5 h-5 transition-transform group-hover:scale-110"
              :class="isActive(item.path) ? 'animate-pulse-glow' : ''"
            />
            <span class="text-[10px] font-medium">{{ item.name }}</span>
          </router-link>
        </div>
      </nav>

      <main class="flex-1 overflow-auto p-6">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>
  </div>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
