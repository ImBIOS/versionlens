---
stream: Cache & Refresh Operations
agent: claude
started: 2026-02-17T12:26:28Z
status: in_progress
---

## Completed

- Created `src/commands/mod.rs`:
  - Module declaration for commands subsystem
  - Exports `cache_commands` and `ui_commands` submodules

- Created `src/commands/cache_commands.rs`:
  - `CacheCommands` struct with command implementations
  - `clear_cache(cache)` - Clears all cached data via FileCache.clear()
  - `check_all_updates()` - Stub for force refreshing all dependencies

- Created `src/commands/ui_commands.rs`:
  - `UiCommands` struct for UI-related commands
  - `toggle_inline()` - Stub for enabling/disabling inline badges
  - `open_registry(package_name, registry)` - Stub for opening package page

- Updated `src/lib.rs`:
  - Added `mod commands;` to include commands module

## Implementation Details

- Clear Cache: Directly calls `cache.clear()` which removes all cache directory contents
- Check All Updates: TODO stub - requires worktree access to find package files
- Commands follow Zed's command registration pattern (stubs ready for full implementation)

## Test Results

Code compiles successfully (`cargo check` passes):
- Only warning is unrelated: unused variable in `src/events/buffer.rs`

## Files Created

- `src/commands/mod.rs` (new)
- `src/commands/cache_commands.rs` (new)
- `src/commands/ui_commands.rs` (new)

## Files Modified

- `src/lib.rs` (added commands module)

## Next Steps

1. Register commands with Zed's command API in the extension initialization
2. Implement `check_all_updates` - requires worktree scanning
3. Implement `toggle_inline` - requires settings/state management
4. Implement `open_registry` - requires URL construction per registry
