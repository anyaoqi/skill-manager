import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'
import './styles/main.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: () => import('./pages/dashboard/Dashboard.vue'),
    },
    {
      path: '/skills',
      name: 'skills',
      component: () => import('./pages/skills/Skills.vue'),
    },
    {
      path: '/skills/:name',
      name: 'skill-detail',
      component: () => import('./pages/skills/SkillDetail.vue'),
    },
    {
      path: '/tools',
      name: 'tools',
      component: () => import('./pages/tools/Tools.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('./pages/settings/Settings.vue'),
    },
  ],
})

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.mount('#app')
