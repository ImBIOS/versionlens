use crate::parsers::types::{Dependency, DependencySection};
use crate::parsers::PackageParser;
use regex::Regex;
use serde_json::Value;

pub struct NpmParser;

impl PackageParser for NpmParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Extract dependencies from each section
        for section in &[
            DependencySection::Dependencies,
            DependencySection::DevDependencies,
            DependencySection::PeerDependencies,
        ] {
            if let Some(sec_deps) = self.extract_section(content, section)? {
                deps.extend(sec_deps);
            }
        }

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "package.json"
    }
}

impl NpmParser {
    fn extract_section(
        &self,
        content: &str,
        section: &DependencySection,
    ) -> Result<Option<Vec<Dependency>>, Box<dyn std::error::Error + Send + Sync>> {
        let section_key = match section {
            DependencySection::Dependencies => "dependencies",
            DependencySection::DevDependencies => "devDependencies",
            DependencySection::PeerDependencies => "peerDependencies",
        };

        // First pass: use regex to find line numbers for each dependency in the section
        let line_numbers = self.find_dependency_line_numbers(content, section_key)?;

        // Second pass: use serde_json to parse the values
        let json: Value = serde_json::from_str(content)?;

        let section_obj = match json.get(section_key) {
            Some(Value::Object(obj)) => obj,
            _ => return Ok(None),
        };

        let mut deps = Vec::new();

        for (name, value) in section_obj {
            let version_specifier = match value {
                Value::String(version) => version.clone(),
                Value::Object(obj) => {
                    // Handle nested version format like { "version": "1.0.0", "registry": "..." }
                    obj.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                }
                _ => String::new(),
            };

            let line_number = line_numbers.get(name).copied().unwrap_or(0);

            deps.push(Dependency {
                name: name.clone(),
                version_specifier,
                line_number,
            });
        }

        Ok(Some(deps))
    }

    fn find_dependency_line_numbers(
        &self,
        content: &str,
        section_key: &str,
    ) -> Result<std::collections::HashMap<String, usize>, Box<dyn std::error::Error + Send + Sync>> {
        let mut line_numbers = std::collections::HashMap::new();

        // Regex to find the section start
        let section_regex = Regex::new(&format!(r#""{}"\s*:"#, section_key))?;
        let dep_regex = Regex::new(r#""([^"]+)"\s*:"#)?;

        let mut in_section = false;
        let mut brace_depth = 0;

        for (line_idx, line) in content.lines().enumerate() {
            let line_number = line_idx + 1; // 1-indexed

            // Check if we've entered the section
            if section_regex.is_match(line) {
                in_section = true;
                brace_depth = 0;
                continue;
            }

            if in_section {
                // Track brace depth
                brace_depth += line.matches('{').count() as i32;
                brace_depth -= line.matches('}').count() as i32;

                // Try to match dependency entries
                if let Some(caps) = dep_regex.captures(line) {
                    if let Some(name) = caps.get(1) {
                        line_numbers.insert(name.as_str().to_string(), line_number);
                    }
                }

                // Exit section when braces are balanced and we see a closing brace
                if brace_depth <= 0 && line.contains('}') {
                    in_section = false;
                }
            }
        }

        Ok(line_numbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_parser_supports_package_json() {
        let parser = NpmParser;
        assert!(parser.supports_file("package.json"));
        assert!(!parser.supports_file("package-lock.json"));
        assert!(!parser.supports_file("Cargo.toml"));
    }

    #[test]
    fn test_npm_parser_parses_dependencies() {
        let parser = NpmParser;
        let content = r#"{
  "name": "test",
  "dependencies": {
    "react": "^18.0.0",
    "lodash": "4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}"#;

        let deps = parser.parse(content).unwrap();
        assert_eq!(deps.len(), 3);

        let react = deps.iter().find(|d| d.name == "react").unwrap();
        assert_eq!(react.version_specifier, "^18.0.0");

        let jest = deps.iter().find(|d| d.name == "jest").unwrap();
        assert_eq!(jest.version_specifier, "^29.0.0");
    }

    #[test]
    fn test_npm_parser_parses_peer_dependencies() {
        let parser = NpmParser;
        let content = r#"{
  "peerDependencies": {
    "react": "^18.0.0"
  }
}"#;

        let deps = parser.parse(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "react");
    }

    #[test]
    fn test_npm_parser_handles_nested_version() {
        let parser = NpmParser;
        let content = r#"{
  "dependencies": {
    "some-package": {
      "version": "1.0.0",
      "registry": "https://registry.npmjs.org"
    }
  }
}"#;

        let deps = parser.parse(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "some-package");
        assert_eq!(deps[0].version_specifier, "1.0.0");
    }
}
