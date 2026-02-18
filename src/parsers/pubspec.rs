use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;
use regex::Regex;

pub struct PubspecParser;

impl PubspecParser {
    pub fn new() -> Self {
        Self
    }
}

impl PackageParser for PubspecParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Pattern: package_name: version (accounting for indentation)
        let dep_line_re = Regex::new(r"^\s*(\w+):\s*(.+)$")?;

        // Pattern for version spec (handles caret, tilde, exact versions)
        let version_re = Regex::new(r"^[\^~>]?\s*(\d+\.\d+\.\d+.*)$")?;

        let mut in_dependencies = false;
        let mut in_dev_dependencies = false;

        for (line_num, line) in content.lines().enumerate() {
            // Calculate leading whitespace
            let leading_spaces = line.len() - line.trim_start().len();
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Check for section headers (at the root level, i.e., no leading spaces)
            if leading_spaces == 0 {
                if trimmed == "dependencies:" {
                    in_dependencies = true;
                    in_dev_dependencies = false;
                    continue;
                } else if trimmed == "dev_dependencies:" || trimmed == "dev-dependencies:" {
                    in_dependencies = false;
                    in_dev_dependencies = true;
                    continue;
                } else if trimmed.ends_with(':') {
                    // Other section headers
                    in_dependencies = false;
                    in_dev_dependencies = false;
                    continue;
                }
            }

            // Only process if we're in a dependencies section
            if !in_dependencies && !in_dev_dependencies {
                continue;
            }

            // Try to match a dependency line
            if let Some(caps) = dep_line_re.captures(trimmed) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let version_part = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                // Skip keys that aren't package names (flutter sdk, git, path refs)
                if name == "flutter" || name == "sdk" || name == "git" || name == "path" {
                    continue;
                }

                // Extract version
                let version_spec = if let Some(vcaps) = version_re.captures(version_part.trim()) {
                    vcaps.get(1).map(|m| m.as_str()).unwrap_or("*").to_string()
                } else if version_part.trim() == "any" || version_part.trim().is_empty() {
                    "*".to_string()
                } else {
                    // Could be a git ref, path, or sdk reference
                    "*".to_string()
                };

                if !name.is_empty() {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: version_spec,
                        line_number: line_num + 1,
                    });
                }
            }
        }

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "pubspec.yaml"
    }
}

impl Default for PubspecParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pubspec() {
        let parser = PubspecParser::new();
        let content = r#"
name: myapp
version: 1.0.0
description: A test app

dependencies:
  flutter:
    sdk: flutter
  http: ^1.0.0
  provider: any
  json_annotation: ^4.8.0
  equatable: ^2.0.5
  get_it: ^7.6.0

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0
  build_runner: ^2.4.0
"#;

        let result = parser.parse(content);
        assert!(result.is_ok());
        let deps = result.unwrap();

        assert!(deps.len() >= 7, "Expected >= 7 deps, got {}", deps.len());
        assert!(deps.iter().any(|d| d.name == "http"));
        assert!(deps.iter().any(|d| d.name == "provider"));
        assert!(deps.iter().any(|d| d.name == "json_annotation"));
        assert!(deps.iter().any(|d| d.name == "equatable"));
        assert!(deps.iter().any(|d| d.name == "get_it"));
        assert!(deps.iter().any(|d| d.name == "flutter_lints"));
        assert!(deps.iter().any(|d| d.name == "build_runner"));
    }

    #[test]
    fn test_supports_file() {
        let parser = PubspecParser::new();
        assert!(parser.supports_file("pubspec.yaml"));
        assert!(!parser.supports_file("pubspec.lock"));
    }
}
