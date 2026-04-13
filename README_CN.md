# Skill Dock

[![Release](https://img.shields.io/github/v/release/anyaoqi/skill-dock?include_prereleases)](https://github.com/anyaoqi/skill-dock/releases)
[![License](https://img.shields.io/github/license/anyaoqi/skill-dock)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)]()

一款跨平台桌面应用，用于统一管理多种 AI 编程助手的 Skills。

**[English](README.md)**

---

## 简介

Skill Dock 提供了一个统一的界面来管理各种 AI 编程助手（如 OpenCode、Cursor、Claude Code、GitHub Copilot 等）的 Skills（提示词/指令）。它允许你：

- 扫描并发现所有 AI 工具中已有的 Skills
- 一键为特定工具启用/禁用 Skills
- 通过符号链接在多个工具间共享 Skills
- 创建带有标准 SKILL.md 模板的新 Skills
- 从任意来源删除 Skills

## 支持的工具

| 工具 | Skills 路径 | 支持 Global Agent |
|------|-------------|-------------------|
| Global Agent | `.agents/skills` | - |
| OpenCode | `.config/opencode/skills` | 是 |
| Cursor | `.cursor/skills` | 是 |
| Cline | `.cline/skills` | 是 |
| Amp | `.amp/skills` | 是 |
| Codex | `.codex/skills` | 是 |
| Kimi Code CLI | `.kimi-code/skills` | 是 |
| Qwen Code | `.qwen/skills` | 是 |
| Warp | `.warp/skills` | 是 |
| Claude Code | `.claude/skills` | 否 |
| Gemini CLI | `.gemini/skills` | 否 |
| Antigravity | `.gemini/antigravity/skills` | 否 |
| GitHub Copilot | `.copilot/skills` | 否 |
| Kiro | `.kiro/skills` | 否 |
| Qoder | `.qoder/skills` | 否 |
| Trae | `.trae/skills` | 否 |
| Trae CN | `.trae-cn/skills` | 否 |
| Windsurf | `.windsurf/skills` | 否 |
| Augment Code | `.augment/skills` | 否 |

> 标记为"支持 Global Agent"的工具可以通过符号链接共享 `.agents/skills` 目录中的 Skills。

## 功能特性

- **Skills 发现**：自动扫描所有配置工具的 Skills 目录
- **统一管理**：在单一界面查看和管理所有 Skills
- **跨工具共享**：一键切换即可将 Skill 启用到多个工具
- **Skill 创建**：创建带有标准 SKILL.md frontmatter 模板的新 Skills
- **文件预览**：在应用内直接查看 Skill 文件内容
- **资源管理器集成**：在系统文件管理器中打开 Skills 目录

## 安装

### 下载安装

从 [GitHub Releases](https://github.com/anyaoqi/skill-dock/releases) 下载最新版本：

- **Windows**: `.msi` 或 `.exe` 安装包
- **macOS**: `.dmg` 或 `.app`
- **Linux**: `.deb`、`.rpm` 或 `.AppImage`

### 从源码构建

前置要求：
- Node.js 20+
- pnpm 9+
- Rust（最新稳定版）

```bash
# 克隆仓库
git clone https://github.com/anyaoqi/skill-dock.git
cd skill-dock

# 安装依赖
pnpm install

# 构建并运行
pnpm tauri dev
```

## 使用指南

### 导航

- **仪表盘**：Skills 和工具概览
- **Skills**：查看、搜索、筛选和管理所有 Skills
- **工具**：配置和查看工具特定设置
- **设置**：应用偏好设置

### 管理 Skills

1. **查看 Skills**：所有发现的 Skills 都会列出，显示其来源和已启用的工具
2. **筛选**：使用分类筛选（全局代理 / 工具专属）或按名称搜索
3. **启用/禁用**：使用工具徽章为特定工具切换 Skills
4. **创建**：点击 "+" 按钮创建新的 Skill
5. **删除**：使用删除图标永久移除 Skill

### Skill 结构

Skills 遵循标准的 SKILL.md 格式：

```
skill-name/
├── SKILL.md          # 主 Skill 定义
├── prompts/          # 提示模板（可选）
├── references/       # 参考文档（可选）
└── scripts/          # 工具脚本（可选）
```

SKILL.md 格式：

```markdown
---
name: skill-name
description: Skill 简介
---

# Skill 名称

详细指令和内容...
```

## 技术栈

- **前端**：Vue 3、TypeScript、UnoCSS、Vue Router、Pinia
- **后端**：Tauri 2 (Rust)
- **构建**：Vite、pnpm

## 开发

```bash
# 开发模式
pnpm dev

# 仅构建前端
pnpm build

# 构建 Tauri 应用
pnpm tauri build
```

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详情请见 [LICENSE](LICENSE) 文件。

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- 所有启发本项目的 AI 编程助手工具