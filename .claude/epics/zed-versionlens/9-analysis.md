---
issue: 9
name: Command Palette
status: in_progress
created: 2026-02-17T12:23:18Z
---

# Analysis: Task 9 - Command Palette

## Parallel: yes

## Streams

### Stream A: Cache & Refresh Operations
Files: src/commands/mod.rs, src/commands/cache_commands.rs
Description: Implement "Check All Updates" (force refresh via registry client) and "Clear Cache" (connect to existing FileCache::clear())

### Stream B: UI Toggle & External Actions
Files: src/commands/mod.rs, src/commands/ui_commands.rs, src/state.rs
Description: Implement "Toggle Inline" (state management) and "Open Registry" (URL builder + Zed browser API)

## Dependencies
- Task 6: Inline Badge UI (complete)
- Task 8: Caching Layer (complete)
