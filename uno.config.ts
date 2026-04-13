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
        bg: 'rgb(var(--cyber-bg) / <alpha-value>)',
        card: 'rgb(var(--cyber-card) / <alpha-value>)',
        border: 'rgb(var(--cyber-border) / <alpha-value>)',
        primary: 'rgb(var(--cyber-primary) / <alpha-value>)',
        secondary: 'rgb(var(--cyber-secondary) / <alpha-value>)',
        success: 'rgb(var(--cyber-success) / <alpha-value>)',
        warning: 'rgb(var(--cyber-warning) / <alpha-value>)',
        error: 'rgb(var(--cyber-error) / <alpha-value>)',
        text: 'rgb(var(--cyber-text) / <alpha-value>)',
        muted: 'rgb(var(--cyber-muted) / <alpha-value>)',
      },
    },
  },
  shortcuts: {
    'cyber-btn': 'px-4 py-2 rounded-lg font-medium transition-all duration-300 cursor-pointer',
    'cyber-btn-primary': 'cyber-btn bg-cyber-primary/20 text-cyber-primary border border-cyber-primary/30 hover:bg-cyber-primary/30 hover:shadow-[0_0_20px_rgb(var(--cyber-primary)/0.3)]',
    'cyber-btn-secondary': 'cyber-btn bg-cyber-secondary/20 text-cyber-secondary border border-cyber-secondary/30 hover:bg-cyber-secondary/30',
    'cyber-card': 'bg-cyber-card/80 backdrop-blur-md border border-cyber-border rounded-xl p-4 transition-all duration-300',
    'cyber-card-hover': 'cyber-card hover:border-cyber-primary/50 hover:shadow-[0_0_30px_rgb(var(--cyber-primary)/0.1)] hover:scale-[1.02]',
    'cyber-input': 'bg-cyber-bg border border-cyber-border rounded-lg px-4 py-2 text-cyber-text focus:outline-none focus:border-cyber-primary/50 transition-all',
    'cyber-neon': 'shadow-[0_0_10px_rgb(var(--cyber-primary)/0.5)]',
    'cyber-neon-secondary': 'shadow-[0_0_10px_rgb(var(--cyber-secondary)/0.5)]',
  },
  rules: [
    ['glass', { 'background': 'rgb(var(--cyber-card) / 0.7)', 'backdrop-filter': 'blur(12px)' }],
  ],
})
