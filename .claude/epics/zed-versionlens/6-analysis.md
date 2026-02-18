---
issue: 6
name: Inline Badge UI
status: in_progress
created: 2026-02-17T11:29:18Z
---

# Analysis: Task 6 - Inline Badge UI

## Parallel: yes

## Streams

### Stream A: UI/Decoration Layer
Files: `src/ui/mod.rs`, `src/ui/decorations.rs`, `src/ui/badges.rs`
Description: Using Zed's decoration/appearance API to render inline badges. Implement color coding (green up-to-date, yellow outdated, red major version diff). Badge styling and positioning.

### Stream B: Version Comparison Logic
Files: `src/version/mod.rs`, `src/version/comparison.rs`
Description: Using semver crate to compare versions. Determine version diff (major, minor, patch). Generate version comparison text ("^1.2.3 → 1.4.0").

### Stream C: Event Handling & Integration
Files: `src/events/mod.rs`, `src/events/buffer.rs`, `src/events/debouncer.rs`
Description: Handle file open events. Active buffer detection. Debounce API calls. Integration with registry client (Task 5) and package parser (Task 4).

## Dependencies
- Tasks 4 and 5 are complete - ready to integrate
- Stream C depends on A and B completing (for integration)
- Streams A and B are independent
