// UnoCSS Configuration
import { defineConfig, presetUno, presetWebFonts, presetIcons } from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
    presetWebFonts({
      provider: 'google',
      fonts: {
        mono: 'JetBrains Mono',
        sans: 'Inter:400,500,600,700',
      },
    }),
    presetIcons({
      scale: 1.2,
      cdn: 'https://esm.sh/',
    }),
  ],
  theme: {
    colors: {
      cyber: {
        bg: '#0B0F17',
        card: '#111827',
        border: '#1F2937',
        primary: '#00F5FF',
        secondary: '#7C3AED',
        success: '#10B981',
        warning: '#F59E0B',
        error: '#EF4444',
        text: '#F9FAFB',
        muted: '#9CA3AF',
      },
    },
  },
  shortcuts: {
    'cyber-btn': 'px-4 py-2 rounded-lg font-medium transition-all duration-300 cursor-pointer',
    'cyber-btn-primary': 'cyber-btn bg-cyber-primary/20 text-cyber-primary border border-cyber-primary/30 hover:bg-cyber-primary/30 hover:shadow-[0_0_20px_rgba(0,245,255,0.3)]',
    'cyber-btn-secondary': 'cyber-btn bg-cyber-secondary/20 text-cyber-secondary border border-cyber-secondary/30 hover:bg-cyber-secondary/30',
    'cyber-card': 'bg-cyber-card/80 backdrop-blur-md border border-cyber-border rounded-xl p-4 transition-all duration-300',
    'cyber-card-hover': 'cyber-card hover:border-cyber-primary/50 hover:shadow-[0_0_30px_rgba(0,245,255,0.1)] hover:scale-[1.02]',
    'cyber-input': 'bg-cyber-bg border border-cyber-border rounded-lg px-4 py-2 text-cyber-text focus:outline-none focus:border-cyber-primary/50 transition-all',
    'cyber-neon': 'shadow-[0_0_10px_rgba(0,245,255,0.5)]',
    'cyber-neon-secondary': 'shadow-[0_0_10px_rgba(124,58,237,0.5)]',
  },
  rules: [
    ['glass', { 'background': 'rgba(17, 24, 39, 0.7)', 'backdrop-filter': 'blur(12px)' }],
  ],
})
