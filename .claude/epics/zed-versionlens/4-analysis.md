---
issue: 4
name: Package Parsing
status: in_progress
created: 2026-02-17T11:11:49Z
---

# Analysis: Task 4 - Package Parsing

## Parallel: yes

## Streams

### Stream A: PackageParser trait + data types (PREREQUISITE)
Files: `src/parsers/mod.rs`, `src/parsers/types.rs`
Description: Define the `PackageParser` trait and shared types (`Dependency { name, version_specifier, line_number }`, `DependencySection` enum). Small - must land first.

### Stream B: npm package.json parser
Files: `src/parsers/npm.rs`
Description: Core tree-sitter JSON parsing for package.json. Note: `zed_extension_api` 0.1.0 may not expose tree-sitter to WASM - may need to use `serde_json` instead.

### Stream C: Secondary format parsers (stubs)
Files: `src/parsers/cargo.rs`, `src/parsers/go.rs`, `src/parsers/pyproject.rs`, `src/parsers/gemfile.rs`, `src/parsers/pubspec.rs`
Description: Regex-based stub parsers implementing the trait from Stream A.

## Dependencies
- Stream A must complete before B and C can start
- B and C are independent of each other
