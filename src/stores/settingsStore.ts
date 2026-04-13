import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings } from '../types'

const STORAGE_KEY = 'skilldock-settings'

function loadSettings(): AppSettings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) {
      return { theme: 'dark', autoScan: true, notifications: true, ...JSON.parse(raw) }
    }
  } catch {
    // ignore parse errors
  }
  return { theme: 'dark', autoScan: true, notifications: true }
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>(loadSettings())

  const updateSettings = (newSettings: Partial<AppSettings>) => {
    settings.value = { ...settings.value, ...newSettings }
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value))
    } catch {
      // ignore storage errors
    }
  }

  return {
    settings,
    updateSettings,
  }
})
