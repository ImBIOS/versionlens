use crate::parsers::types::Dependency;
use crate::parsers::PackageParser;
use regex::Regex;

pub struct GoParser;

impl GoParser {
    pub fn new() -> Self {
        Self
    }
}

impl PackageParser for GoParser {
    fn parse(&self, content: &str) -> Result<Vec<Dependency>, Box<dyn std::error::Error + Send + Sync>> {
        let mut deps = Vec::new();

        // Regex to match individual require lines (either in block or after require keyword)
        // For lines in block: github.com/some/pkg v1.2.3
        // For single lines: require golang.org/x/net v0.17.0
        let require_line_re = Regex::new(r"(?:require\s+)?([^\s]+)\s+v?(\d+\.\d+\.\d+.*)")?;

        // Track if we're inside a require block
        let mut in_require_block = false;

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Detect require block start
            if trimmed == "require(" || trimmed == "require (" {
                in_require_block = true;
                continue;
            }

            // Detect require block end
            if in_require_block && trimmed == ")" {
                in_require_block = false;
                continue;
            }

            // Parse require lines (either in block or single line)
            if let Some(caps) = require_line_re.captures(trimmed) {
                let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let version = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                if !name.is_empty() && !version.is_empty() && name != "go" {
                    deps.push(Dependency {
                        name: name.to_string(),
                        version_specifier: version.to_string(),
                        line_number: line_num + 1,
                    });
                }
            }
        }

        Ok(deps)
    }

    fn supports_file(&self, filename: &str) -> bool {
        filename == "go.mod"
    }
}

impl Default for GoParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_go_mod() {
        let parser = GoParser::new();
        let content = r#"module example.com/myapp

go 1.21

require (
    github.com/some/pkg v1.2.3
    golang.org/x/text v0.14.0
)

require golang.org/x/net v0.17.0
"#;

        let result = parser.parse(content);
        assert!(result.is_ok());
        let deps = result.unwrap();

        assert!(deps.len() >= 3, "Expected >= 3 deps, got {}", deps.len());
        assert!(deps.iter().any(|d| d.name == "github.com/some/pkg"));
        assert!(deps.iter().any(|d| d.name == "golang.org/x/text"));
        assert!(deps.iter().any(|d| d.name == "golang.org/x/net"));
    }

    #[test]
    fn test_supports_file() {
        let parser = GoParser::new();
        assert!(parser.supports_file("go.mod"));
        assert!(!parser.supports_file("Cargo.toml"));
    }
}
