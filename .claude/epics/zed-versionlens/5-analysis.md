---
issue: 5
name: npm Registry Client
status: in_progress
created: 2026-02-17T11:11:49Z
---

# Analysis: Task 5 - npm Registry Client

## Parallel: no

## Streams

### Stream A: npm Registry Client Implementation
Files: `src/registry/mod.rs`, `src/registry/npm.rs`
Description: Implement HTTP client using `zed_extension_api::http_client` to query npm registry, parse response, handle errors.

## Technical Notes
- API: GET https://registry.npmjs.org/{package}
- Response: {"dist-tags": {"latest": "x.y.z"}, "versions": {...}}
- Use existing `serde_json` dependency for parsing
- Handle: timeout, 404, network errors gracefully
