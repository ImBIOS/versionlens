---
name: zed-versionlens
description: Multi-platform Zed extension that displays version information and update availability for dependencies in package files
status: backlog
created: 2026-02-11T04:45:11Z
updated: 2026-02-11T04:58:46Z
---

# PRD: Zed VersionLens

## Executive Summary

VersionLens is a Zed editor extension that displays version information for dependencies directly in the editor. It shows current versions alongside latest available versions from package registries, enabling developers to quickly identify outdated dependencies. The extension supports multiple ecosystems (npm, Cargo, Go, Python, Ruby, Dart) and provides both inline visual indicators and a detailed dashboard view.

**Value Proposition:** Reduce context switching and manual version checking by bringing dependency intelligence directly into the development workflow.

## Problem Statement

Developers working with multiple package managers face several pain points:

1. **Context switching:** Checking for dependency updates requires leaving the editor to visit registry websites (npmjs.com, crates.io, etc.) or running CLI commands
2. **Visibility:** No immediate indication of which dependencies are outdated while reviewing package files
3. **Multi-language complexity:** Projects using multiple languages (e.g., frontend + backend) require different tools and workflows for each ecosystem
4. **Security awareness:** Outdated dependencies with known vulnerabilities may go unnoticed without proactive checking

**Why now:** Zed's extension ecosystem is growing, and VS Code's VersionLens extension has proven this workflow is valuable. A native Zed implementation will perform better and integrate more deeply with Zed's architecture.

## User Stories

### Primary Personas

1. **Full-stack Developer (Alex):** Works on web projects with Node.js frontend and Rust backend
2. **Language Specialist (Sam):** Primarily works in a single ecosystem (e.g., Python or Go)
3. **Security-conscious Developer (Jordan):** Needs to stay aware of dependency vulnerabilities

### User Journeys

**Story 1: Quick outdated dependency check**
> As Alex, I want to see which dependencies are outdated when I open package.json, so I can decide whether to update them before starting work.

**Acceptance Criteria:**
- Inline indicators appear within 2 seconds of opening a supported file
- Outdated versions are visually distinct from up-to-date ones
- Current and latest version are both visible

**Story 2: Detailed version information**
> As Sam, I want to see detailed version information including pre-releases and release dates, so I can make informed decisions about updating.

**Acceptance Criteria:**
- Command palette action opens detail panel
- Shows version history for selected dependency
- Displays release dates and links to changelogs

**Story 3: Security awareness**
> As Jordan, I want to be alerted when a dependency has known security vulnerabilities, so I can prioritize critical updates.

**Acceptance Criteria:**
- Vulnerable dependencies highlighted with warning indicator
- Severity level is displayed
- Links to security advisories are provided

## Requirements

### Functional Requirements

#### FR1: File Format Support
The extension MUST parse and display version information for:
- `package.json` (npm/Yarn/pnpm)
- `Cargo.toml` (Rust)
- `go.mod` (Go modules)
- `pyproject.toml` (Python)
- `Gemfile` (Ruby)
- `pubspec.yaml` (Dart/Flutter)

#### FR2: Registry Integration
The extension MUST query official registries for latest versions:
- npm Registry (https://registry.npmjs.org)
- crates.io (https://crates.io)
- Go Proxy (https://proxy.golang.org)
- PyPI (https://pypi.org)
- RubyGems (https://rubygems.org)
- pub.dev (https://pub.dev)

#### FR3: Inline Visual Indicators
The extension MUST provide configurable visual indicators:
- Inline badge text showing version comparison (e.g., `^1.2.3 → 1.4.0`)
- Color highlighting for outdated versions
- Gutter icons on lines with outdated dependencies
- All indicators must be togglable via settings

#### FR4: Automatic Version Checking
The extension MUST automatically check for updates when:
- A supported package file is opened
- A package file is modified (with debouncing)

#### FR5: Command Palette Actions
The extension MUST provide these commands:
- `VersionLens: Check All Updates` - Manual refresh for current file
- `VersionLens: Show Details` - Show full version history for selected dependency
- `VersionLens: Ignore Dependency` - Add to ignore list
- `VersionLens: Clear Cache` - Clear version cache
- `VersionLens: Toggle Inline Indicators` - Enable/disable inline display
- `VersionLens: Open Registry Page` - Open dependency page on registry

#### FR6: Dashboard Panel
The extension SHOULD provide a panel showing:
- All outdated dependencies in current workspace
- Severity/urgency indicators
- Quick actions (update, ignore, view details)

#### FR7: Caching
The extension MUST cache version information:
- Location: `~/.config/zed/extensions/versionlens/cache/`
- Default TTL: 24 hours
- Configurable per-registry
- Cache key: `{registry}@{package-name}`

#### FR8: Ignore List
The extension MUST support ignoring specific dependencies:
- Per-project ignore list (`.versionlens-ignore`)
- Global ignore list in settings
- Patterns support (e.g., `@types/*`)

### Non-Functional Requirements

#### NFR1: Performance
- Inline indicators appear within 2 seconds of file open
- Cache hits return in <100ms
- No blocking UI operations during version fetching

#### NFR2: Reliability
- Graceful degradation when registries are unavailable
- No extension crashes due to malformed package files
- Network timeouts handled appropriately

#### NFR3: Security
- No credentials sent to third-party services
- Local caching only (no cloud services)
- Verifiable HTTPS connections to registries

#### NFR4: Compatibility
- Support Zed's latest stable release
- Support Linux, macOS, Windows
- No external dependencies beyond Rust standard library + Zed API

## Success Criteria

### Primary Metrics
- **Adoption:** 100+ active users within 3 months of release
- **Performance:** 95% of version checks complete within 2 seconds
- **Reliability:** <1% crash rate (measured via crash reports)

### Secondary Metrics
- **Engagement:** Average 5+ version checks per active user per day
- **Satisfaction:** 4.0+ star rating on Zed extensions marketplace
- **Coverage:** All 6 listed ecosystems functional in v1.0

## Constraints & Assumptions

### Constraints
- **Zed Extension API:** Must work within Zed's current extension capabilities (WASM sandbox)
- **No external services:** Cannot depend on third-party API proxies or hosted services
- **Open source:** Must be released under permissive license (MIT)

### Assumptions
- Zed's extension API will support process spawning for the companion binary
- All target registries will maintain their current API structures
- Users have internet connectivity for version checks (offline mode is out of scope for v1)

### Technical Constraints
- **WASM HTTP Support:** `zed_extension_api::http_client` provides full HTTP/HTTPS support from WASM
- **No companion binary needed:** All registry queries can be made directly from the extension

## Out of Scope

### Explicitly NOT building in v1.0:
- **Automatic dependency updates** (read-only for now)
- **Offline mode** (requires network connectivity)
- **Monorepo workspace support** (single package file focus initially)
- **Vulnerability scanning integration** (version comparison only, not security advisories)
- **License compliance checking** (not tracking license information)
- **Biome/Prettier integration** (Zed-only for now)
- **Custom registry support** (official registries only)
- **Dependency graph visualization** (too complex for initial release)

## Dependencies

### Internal Dependencies
- Zed Extension API stability (specifically `zed_extension_api::http_client`)

### External Dependencies
- **Registry APIs availability:** All target registries must be operational
- **Rust ecosystem:** crates.io for serde (used within extension, no external runtime deps)

### Team Dependencies
- None (single-dev project)

## Architecture Overview

### Components

```
┌─────────────────────────────────────────────────────────────┐
│                        Zed Editor                           │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │           VersionLens Extension (WASM)                │  │
│  │                                                       │  │
│  │  • UI rendering (inline badges, gutter icons)        │  │
│  │  • Inline views                                       │  │
│  │  • Command palette actions                           │  │
│  │  • Package file parsing (package.json, Cargo.toml)   │  │
│  │  • HTTP client (zed_extension_api::http_client)      │  │
│  │  • Registry API calls (npm, crates.io, etc.)         │  │
│  │  • Local file caching                                │  │
│  └───────────────────────────────────────────────────────┘  │
│                           │                                 │
│                           ▼                                 │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Registry APIs (HTTPS)                    │  │
│  │  • registry.npmjs.org                                 │  │
│  │  • crates.io                                          │  │
│  │  • proxy.golang.org                                   │  │
│  │  • pypi.org                                           │  │
│  │  • rubygems.org                                       │  │
│  │  • pub.dev                                            │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                               │
│  ┌───────────────────────────────────────────────────────┐  │
│  │         Local File Cache (optional persistence)        │  │
│  │         ~/.config/zed/extensions/versionlens/cache/    │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### HTTP Client Usage

```rust
use zed_extension_api::http_client::{HttpRequest, HttpMethod};

let request = HttpRequest {
    method: HttpMethod::Get,
    url: "https://registry.npmjs.org/react".to_string(),
    headers: vec![],
    body: None,
    redirect_policy: Default::default(),
};

let response = request.fetch()?;
```

## Implementation Phases

### Phase 1: MVP (Milestone 1)
- HTTP client integration for npm registry (https://registry.npmjs.org)
- Basic inline indicators for package.json
- File caching (extension folder)
- Core commands (refresh, clear cache)

### Phase 2: Multi-ecosystem (Milestone 2)
- Add Cargo.toml, go.mod support
- Registry parsers for crates.io, Go Proxy
- Improved error handling

### Phase 3: Feature Complete (Milestone 3)
- All 6 ecosystems supported
- Dashboard panel
- Ignore list functionality
- Command palette actions complete

### Phase 4: Polish (Milestone 4)
- Settings/configuration UI
- Performance optimization
- Documentation and examples
- Beta testing and feedback

## Open Questions

### ✅ RESOLVED: HTTP from WASM
**Question:** Does Zed's WASM sandbox support outbound HTTPS?

**Answer:** YES - `zed_extension_api::http_client` provides full HTTP/HTTPS support directly from WASM extensions. The module includes:
- `HttpRequest` - struct for building requests
- `HttpResponse` - struct for handling responses
- `fetch()` - function for executing requests
- `fetch_stream()` - function for streaming responses
- Full support for headers, methods (GET/POST/etc.), redirect policies

**Impact:** No companion binary needed. Simpler architecture, better performance.

**Sources:**
- https://docs.rs/zed_extension_api/latest/zed_extension_api/http_client/index.html
- https://docs.rs/zed_extension_api/latest/zed_extension_api/http_client/struct.HttpRequest.html

---

### ✅ RESOLVED: Process Spawning (Alternative Approach)
**Question:** Can Zed extensions spawn companion binaries?

**Answer:** YES - The `process:exec` capability grants extensions the ability to invoke commands using `zed_extension_api::process::Command`. This is user-configurable via `granted_extension_capabilities` settings.

**Impact:** Companion binary approach would work if needed for complex parsing, but is not required since HTTP is directly available.

**Source:** https://zed.dev/docs/extensions/capabilities#process:exec

---

### 🔍 PENDING: Persistence Layer
**Question:** Where should cache be stored cross-platform?

**Options:**
- Extension folder: `~/.config/zed/extensions/versionlens/cache/`
- XDG standard locations on Linux
- AppData on Windows

**Decision Needed:** Investigate if Zed provides an extension storage API for persistent data storage.

**Workaround for Now:** Use a `cache/` subdirectory within the extension folder (cross-platform compatible).
