---
name: add-and-remove-npm-scripts-and-dependencies
description: Workflow command scaffold for add-and-remove-npm-scripts-and-dependencies in skill-manager.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /add-and-remove-npm-scripts-and-dependencies

Use this workflow when working on **add-and-remove-npm-scripts-and-dependencies** in `skill-manager`.

## Goal

Add or remove npm scripts and dependencies to address development workflow issues or feature needs.

## Common Files

- `package.json`
- `pnpm-lock.yaml`
- `src-tauri/Cargo.lock`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Add or remove script in package.json
- Add or remove dependency in package.json
- Update lock files (pnpm-lock.yaml, Cargo.lock) accordingly
- Commit all related changes together

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.