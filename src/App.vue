<script setup lang="ts">
import { useRoute } from 'vue-router'
import {
  LayoutDashboard,
  Boxes,
  Plug,
  Settings,
  Search,
  Bell,
  Command,
  ArrowRightLeft,
} from 'lucide-vue-next'

const route = useRoute()

const navItems = [
  { path: '/', name: 'Dashboard', icon: LayoutDashboard },
  { path: '/skills', name: 'Skills', icon: Boxes },
  { path: '/tools', name: 'Tools', icon: Plug },
  { path: '/sync', name: 'Sync', icon: ArrowRightLeft },
  { path: '/settings', name: 'Settings', icon: Settings },
]

const isActive = (path: string) => {
  if (path === '/') return route.path === '/'
  return route.path.startsWith(path)
}
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-cyber-bg overflow-hidden">
    <header class="h-14 flex items-center justify-between px-4 border-b border-cyber-border glass">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-cyber-primary to-cyber-secondary flex items-center justify-center">
          <Command class="w-5 h-5 text-white" />
        </div>
        <span class="text-lg font-semibold gradient-text">SkillDock</span>
      </div>
      
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-cyber-card border border-cyber-border">
          <Search class="w-4 h-4 text-cyber-muted" />
          <input 
            type="text" 
            placeholder="Search skills..." 
            class="bg-transparent border-none outline-none text-sm text-cyber-text placeholder-cyber-muted w-48"
          />
          <span class="text-xs text-cyber-muted border border-cyber-border px-1.5 py-0.5 rounded">⌘K</span>
        </div>
        <button class="p-2 rounded-lg hover:bg-cyber-card transition-colors">
          <Bell class="w-5 h-5 text-cyber-muted" />
        </button>
      </div>
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
