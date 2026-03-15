import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({
    theme: 'dark',
    autoSync: true,
    syncInterval: 5,
    notifications: true,
    customToolPaths: {},
  })

  const updateSettings = (newSettings: Partial<AppSettings>) => {
    settings.value = { ...settings.value, ...newSettings }
  }

  const setCustomToolPath = (toolId: string, path: string) => {
    settings.value.customToolPaths[toolId] = path
  }

  return {
    settings,
    updateSettings,
    setCustomToolPath,
  }
})
