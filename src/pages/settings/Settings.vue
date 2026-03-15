<script setup lang="ts">
import { useSettingsStore } from '../../stores/settingsStore'
import {
  Settings,
  Bell,
  RefreshCw,
  Moon,
  Sun,
  Monitor,
} from 'lucide-vue-next'

const settingsStore = useSettingsStore()
</script>

<template>
  <div class="space-y-6 max-w-2xl">
    <div>
      <h1 class="text-2xl font-bold text-cyber-text">Settings</h1>
      <p class="text-cyber-muted mt-1">Configure SkillDock preferences</p>
    </div>

    <div class="space-y-4">
      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">Appearance</h2>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <Moon class="w-5 h-5 text-cyber-muted" />
              <div>
                <p class="text-cyber-text">Theme</p>
                <p class="text-xs text-cyber-muted">Choose your preferred theme</p>
              </div>
            </div>
            <div class="flex items-center gap-1 bg-cyber-bg rounded-lg p-1">
              <button 
                class="p-2 rounded transition-colors"
                :class="settingsStore.settings.theme === 'dark' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              >
                <Moon class="w-4 h-4" />
              </button>
              <button 
                class="p-2 rounded transition-colors"
                :class="settingsStore.settings.theme === 'light' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              >
                <Sun class="w-4 h-4" />
              </button>
              <button 
                class="p-2 rounded transition-colors"
                :class="settingsStore.settings.theme === 'system' ? 'bg-cyber-primary/20 text-cyber-primary' : 'text-cyber-muted'"
              >
                <Monitor class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">Sync</h2>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <RefreshCw class="w-5 h-5 text-cyber-muted" />
              <div>
                <p class="text-cyber-text">Auto Sync</p>
                <p class="text-xs text-cyber-muted">Automatically sync skills periodically</p>
              </div>
            </div>
            <button 
              class="w-12 h-6 rounded-full transition-colors relative"
              :class="settingsStore.settings.autoSync ? 'bg-cyber-primary' : 'bg-cyber-border'"
              @click="settingsStore.updateSettings({ autoSync: !settingsStore.settings.autoSync })"
            >
              <span 
                class="absolute top-1 w-4 h-4 rounded-full bg-white transition-transform"
                :class="settingsStore.settings.autoSync ? 'left-7' : 'left-1'"
              ></span>
            </button>
          </div>

          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <Settings class="w-5 h-5 text-cyber-muted" />
              <div>
                <p class="text-cyber-text">Sync Interval</p>
                <p class="text-xs text-cyber-muted">How often to sync (minutes)</p>
              </div>
            </div>
            <select 
              class="cyber-input w-24"
              :value="settingsStore.settings.syncInterval"
              @change="settingsStore.updateSettings({ syncInterval: Number(($event.target as HTMLSelectElement).value) })"
            >
              <option :value="1">1 min</option>
              <option :value="5">5 min</option>
              <option :value="15">15 min</option>
              <option :value="30">30 min</option>
              <option :value="60">1 hour</option>
            </select>
          </div>
        </div>
      </div>

      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">Notifications</h2>
        
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <Bell class="w-5 h-5 text-cyber-muted" />
            <div>
              <p class="text-cyber-text">Push Notifications</p>
              <p class="text-xs text-cyber-muted">Receive notifications for sync status</p>
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

      <div class="cyber-card">
        <h2 class="text-lg font-semibold text-cyber-text mb-4">About</h2>
        
        <div class="space-y-3">
          <div class="flex items-center justify-between py-2 border-b border-cyber-border">
            <span class="text-cyber-muted">Version</span>
            <span class="text-cyber-text">0.1.0</span>
          </div>
          <div class="flex items-center justify-between py-2 border-b border-cyber-border">
            <span class="text-cyber-muted">Framework</span>
            <span class="text-cyber-text">Tauri + Vue</span>
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
