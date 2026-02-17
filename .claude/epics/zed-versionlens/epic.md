---
name: zed-versionlens
status: backlog
created: 2026-02-17T08:11:46Z
updated: 2026-02-17T08:11:46Z
progress: 0%
base_branch: ccpm-explore
prd: .claude/prds/zed-versionlens.md
github: https://github.com/ImBIOS/versionlens/issues/1
---

# Epic: zed-versionlens

## Overview
A Zed editor extension that displays version information for dependencies directly inline. Uses Rust/WASM with `zed_extension_api::http_client` for direct registry queries. No companion binary needed - all functionality runs in the extension WASM sandbox.

## Architecture Decisions

- **Language:** Rust (Zed extension standard)
- **HTTP Client:** `zed_extension_api::http_client` - direct HTTPS calls from WASM
- **Caching:** Local file cache in extension folder (`cache/`) with 24h TTL
- **Parser Approach:** tree-sitter for JSON (package.json), regex/manual for TOML/YAML - Zed has built-in tree-sitter support!
- **UI:** Inline badges via Zed's decoration API, command palette actions
- **Reference:** [zed-industries/package-version-server](https://github.com/zed-industries/package-version-server) - npm registry API format, version parsing

## Technical Approach

### Core Components
1. **Package Parser** - Parses package.json, Cargo.toml, go.mod, pyproject.toml, Gemfile, pubspec.yaml
2. **Registry Client** - HTTP client for each registry (npm, crates.io, PyPI, Go Proxy, RubyGems, pub.dev)
3. **Cache Manager** - File-based caching with TTL
4. **UI Decorator** - Inline version badges, gutter icons
5. **Command Handler** - Command palette actions

### Registry API Patterns
- npm: `GET /{package}` → returns `{"dist-tags": {"latest": "x.y.z"}, "versions": {...}}`
- crates.io: `GET /api/v1/crates/{name}` → returns `{"crate": {"max_version": "x.y.z"}}`
- PyPI: `GET /pypi/{package}/json` → returns `{"info": {"version": "x.y.z"}}`
- Go: `GET /{module}/@v/list` (proxy.golang.org)
- RubyGems: `GET /gems/{gem}.json` → returns `{"version": "x.y.z"}`
- pub.dev: `GET /api/packages/{package}` → returns `{"latest": {"version": "x.y.z"}}`

## Implementation Strategy

### Phase 1: MVP (npm only)
- Set up extension structure with Cargo.toml
- Implement npm registry client (follow package-version-server pattern)
- Parse package.json using tree-sitter (Zed's built-in)
- Basic inline badge display
- Command: "Check Updates"

### Phase 2: Multi-registry
- Add Cargo.toml, go.mod parsers
- Implement crates.io, Go Proxy clients
- Error handling for unavailable registries

### Phase 3: Full ecosystem
- Add Python, Ruby, Dart support
- Cache layer with TTL
- Ignore list functionality

### Phase 4: Polish
- Settings/configuration
- Dashboard panel (if API supports)
- Performance optimization

## Task Breakdown Preview

- [ ] **T1: Extension Setup** - Cargo.toml, extension.toml, basic structure
- [ ] **T2: Package Parsing** - Parse all 6 file formats for dependencies
- [ ] **T3: npm Registry Client** - HTTP client + response parsing for npm
- [ ] **T4: Inline Badge UI** - Decoration API for version badges
- [ ] **T5: Multi-registry Support** - Add crates.io, PyPI, Go, Ruby, pub.dev
- [ ] **T6: Caching Layer** - File cache with TTL per registry
- [ ] **T7: Command Palette** - Refresh, clear cache, ignore, details commands
- [ ] **T8: Settings & Polish** - Configuration UI, performance tuning

## Tasks Created
- [ ] #3 - Extension Setup (parallel: true)
- [ ] #4 - Package Parsing (parallel: true)
- [ ] #5 - npm Registry Client (parallel: true)
- [ ] #6 - Inline Badge UI (parallel: true)
- [ ] #7 - Multi-registry Support (parallel: true)
- [ ] #8 - Caching Layer (parallel: true)
- [ ] #9 - Command Palette (parallel: true)
- [ ] #10 - Settings & Polish (parallel: true)

Total tasks: 8
Parallel tasks: 8
Sequential tasks: 0
Estimated total effort: 30 hours

## Dependencies

- `zed_extension_api` - HTTP client, file system, settings
- `semver` or `semver_rs` - for version comparison
- Registry APIs (must be reachable)

## Success Criteria (Technical)

- Extension loads without errors in Zed
- npm package.json shows version badges within 2 seconds
- Cache reduces redundant network calls
- Graceful degradation when registry unavailable
- Works on Linux, macOS, Windows
