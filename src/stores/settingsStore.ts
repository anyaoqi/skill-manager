import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({
    theme: 'dark',
    autoScan: true,
    notifications: true,
  })

  const updateSettings = (newSettings: Partial<AppSettings>) => {
    settings.value = { ...settings.value, ...newSettings }
  }

  return {
    settings,
    updateSettings,
  }
})
