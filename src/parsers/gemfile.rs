use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;
use regex::Regex;

pub struct GemfileParser;

impl GemfileParser {
    pub fn new() -> Self {
        Self
    }
}

impl PackageParser for GemfileParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Pattern 1: gem 'name', 'version'
        let gem_quoted_re = Regex::new(r#"gem\s+['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]"#)?;

        // Pattern 2: gem 'name', '~> version' (pessimistic version)
        let gem_pessimistic_re = Regex::new(r#"gem\s+['"]([^'"]+)['"]\s*,\s*['"]([~^]\s*\d+\.\d+(\.\d+)?)"#)?;

        // Pattern 3: gem 'name' (no version specified)
        let gem_no_version_re = Regex::new(r#"gem\s+['"]([^'"]+)['"]\s*(?:,|\s*)$"#)?;

        // Pattern 4: gem name, version (without quotes)
        let gem_unquoted_re = Regex::new(r#"gem\s+(\w+)\s*,\s*['"]([^'"]+)['"]"#)?;

        // Find all lines
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip comments
            if line.starts_with('#') {
                continue;
            }

            // Try pattern 1: gem 'name', 'version'
            if let Some(caps) = gem_quoted_re.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let version = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                if !name.is_empty() {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: version.to_string(),
                        line_number: line_num + 1,
                    });
                    continue;
                }
            }

            // Try pattern 2: pessimistic version
            if let Some(caps) = gem_pessimistic_re.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let version = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                if !name.is_empty() && !deps.iter().any(|d| d.name == name) {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: version.to_string(),
                        line_number: line_num + 1,
                    });
                    continue;
                }
            }

            // Try pattern 4: unquoted name
            if let Some(caps) = gem_unquoted_re.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let version = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                if !name.is_empty() && !deps.iter().any(|d| d.name == name) {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: version.to_string(),
                        line_number: line_num + 1,
                    });
                    continue;
                }
            }

            // Try pattern 3: no version (only if we haven't added this gem)
            if let Some(caps) = gem_no_version_re.captures(line) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");

                if !name.is_empty() && !deps.iter().any(|d| d.name == name) {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: "*".to_string(),
                        line_number: line_num + 1,
                    });
                }
            }
        }

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "Gemfile"
    }
}

impl Default for GemfileParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gemfile() {
        let parser = GemfileParser::new();
        let content = r#"
source 'https://rubygems.org'

gem 'rails', '~> 7.0'
gem 'puma', '>= 5.0'
gem 'redis'
gem "sidekiq", "^6.5"
gem 'debug', '~> 1.8', platforms: :ruby

# This is a comment
# gem 'ignored'
"#;

        let result = parser.parse(content);
        assert!(result.is_ok());
        let deps = result.unwrap();

        assert!(deps.len() >= 5);
        assert!(deps.iter().any(|d| d.name == "rails"));
        assert!(deps.iter().any(|d| d.name == "puma"));
        assert!(deps.iter().any(|d| d.name == "redis"));
        assert!(deps.iter().any(|d| d.name == "sidekiq"));
        assert!(deps.iter().any(|d| d.name == "debug"));
    }

    #[test]
    fn test_supports_file() {
        let parser = GemfileParser::new();
        assert!(parser.supports_file("Gemfile"));
        assert!(!parser.supports_file("Gemfile.lock"));
    }
}
